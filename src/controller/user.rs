use super::*;
use crate::data::{
    activation as activation_model, invitation as invitation_model, rank as rank_model,
    torrent_status as torrent_status_model, user as user_model, user_info as user_info_model,
};

static ALLOWED_AVATAR_EXTENSION: [&str; 4] = ["jpg", "jpeg", "png", "webp"];

#[post("/add_user")]
async fn add_user(data: web::Json<SignUpRequest>, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let user = data.into_inner();
    let mut allowed = false;
    let mut code = None;

    match parse_email(&user.email) {
        Some(_email) => {
            #[cfg(feature = "email-restriction")]
            if user.invite_code.is_none()
                && ALLOWED_DOMAIN.read().await.get(&_email.domain).is_some()
            {
                allowed = true;
            }
        }
        None => {
            return Ok(
                HttpResponse::BadRequest().json(GeneralResponse::from_err("invalid email address"))
            )
        }
    }
    if let Some(str) = user.invite_code {
        let mut ret = invitation_model::find_invitation_by_code(&client, &str).await?;
        if !ret.is_empty() {
            if ret[0].usage {
                return Ok(HttpResponse::Ok()
                    .json(GeneralResponse::from_err("invitation code already taken")));
            }
            code = Some(ret.pop().unwrap());
            allowed = true;
        }
    }
    if !allowed {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("not allowed to register")));
    }
    let check = user_model::check_existence(&client, &user.email, &user.username).await?;
    if !check.is_empty() {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err(&format!(
            "{} already taken",
            check
        ))));
    }

    let passkey = generate_passkey(&user.username)?;
    let new_user = user_model::add_user(
        &client,
        &user.email,
        &user.username,
        &hash_password(&user.password)?,
        &passkey,
    )
    .await?;
    if code.is_some() {
        let true_code = code.unwrap();
        user_info_model::add_invitor_by_name(
            &client,
            new_user.id,
            true_code.sender,
            &true_code.code,
        )
        .await?;
    }
    Ok(HttpResponse::Ok().json(new_user.to_json()))
}

#[post("/login")]
async fn login(data: web::Json<LoginRequest>, client: web::Data<sqlx::PgPool>) -> HttpResult {
    use chrono::{Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};

    let secret = CONFIG.secret_key.as_bytes();

    let validation = user_model::find_validation_by_name(&client, &data.username)
        .await?
        .pop();
    if validation.is_none() {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("password not match")));
    }
    let mut val = validation.unwrap();
    if !verify_password(&data.password, &val.password)? {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("password not match")));
    }
    if !val.activated {
        let msg = format!("account with id {} not activated yet", val.id);
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err(&msg)));
    }

    user_info_model::update_activity_by_name(&client, &data.username).await?;
    let current_rank = rank_model::find_rank_by_username(&client, &data.username).await?;
    // TODO: Auto update role in jwt
    if current_rank.next.is_some() {
        let next_rank = rank_model::find_rank_by_id(&client, current_rank.next.unwrap()).await?;
        let info = user_info_model::find_user_info_by_name_mini(&client, &data.username).await?;
        let current = Utc::now().timestamp();
        let before = info.registertime.timestamp();
        if info.upload > next_rank.upload && current - before > next_rank.age {
            let roles = next_rank.role;
            for role in roles {
                user_model::add_role_by_id(&client, val.id, (role % 32) as i32).await?;
            }
            user_info_model::update_rank_by_name(&client, &data.username, &next_rank.name).await?;
            val = user_model::find_validation_by_name(&client, &data.username)
                .await?
                .pop()
                .unwrap();
        }
    }
    let claim = Claim {
        sub: val.username,
        role: val.role,
        exp: (Utc::now() + Duration::days(3)).timestamp(),
    };
    let tokens = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret),
    )
    .unwrap();
    Ok(HttpResponse::Ok().json(tokens.to_json()))
}

#[post("/personal_info_update")]
async fn personal_info_update(
    data: web::Json<InfoWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let username = get_name_in_token(&req)?;
    let level: user_info_model::Level = data.privacy.try_into()?;

    user_info_model::update_privacy_by_name(&client, &username, level).await?;
    user_info_model::update_other_by_name(&client, &username, &data.info).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[post("/upload_avatar")]
async fn upload_avatar(
    mut payload: actix_multipart::Multipart,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    use futures::{StreamExt, TryStreamExt};

    let username = get_name_in_token(&req)?;

    if let Ok(Some(mut file)) = payload.try_next().await {
        let content_type = file
            .content_disposition()
            .ok_or_else(|| Error::OtherError("incomplete file".to_string()))?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| "incomplete file".to_string())?;
        let cleaned_name = sanitize_filename::sanitize(&filename);

        let suffix = cleaned_name
            .rfind('.')
            .ok_or_else(|| Error::OtherError("missing filename extension".to_string()))?;
        let ext = cleaned_name[suffix + 1..].to_ascii_lowercase();
        if ALLOWED_AVATAR_EXTENSION
            .iter()
            .find(|x| x == &&ext.as_str())
            .is_none()
        {
            return Ok(HttpResponse::UnsupportedMediaType()
                .json(GeneralResponse::from_err("must be jpg or png or webp")));
        }

        let mut buf: Vec<u8> = vec![];
        while let Some(chunk) = file.next().await {
            let data = chunk.map_err(error_string)?;
            buf.append(&mut data.to_vec());
        }
        let encoded_avatar = base64::encode(buf);
        user_info_model::update_avatar_by_name(&client, &username, &encoded_avatar).await?;
    }

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[get("show_user")]
async fn show_user(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    let current_user = claim.sub;

    let data = deserialize_from_req!(req, UsernameWrapper);
    let name = data.username.unwrap_or(current_user.clone());
    let mut ret = user_info_model::find_user_info_by_name(&client, &name).await?;
    if name == current_user {
        Ok(HttpResponse::Ok().json(ret.to_json()))
    } else if ret.privacy == 1 && is_no_permission_to_users(claim.role) {
        Err(Error::NoPermission)
    } else {
        ret.passkey = "".to_string();
        Ok(HttpResponse::Ok().json(ret.to_json()))
    }
}

#[get("/show_torrent_status")]
async fn show_torrent_status(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let current_user = get_name_in_token(&req)?;
    let data = deserialize_from_req!(req, UsernameWrapper);
    let name = data.username.unwrap_or(current_user);
    let user = user_info_model::find_user_info_by_name_mini(&client, &name).await?;

    let downloading = torrent_status_model::find_downloading_torrent(&client, user.id).await?;
    let uploading = torrent_status_model::find_uploading_torrent(&client, user.id).await?;
    let finished = torrent_status_model::find_finished_torrent(&client, user.id).await?;
    let unfinished = torrent_status_model::find_unfinished_torrent(&client, user.id).await?;
    let ret = TorrentStatusByUser {
        uploading,
        downloading,
        finished,
        unfinished,
    };

    Ok(HttpResponse::Ok().json(ret.to_json()))
}

#[post("/reset_password")]
async fn reset_password(
    data: web::Json<PasswordWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let username = get_name_in_token(&req)?;
    user_model::update_password_by_username(&client, &username, &hash_password(&data.password)?)
        .await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[get("/reset_passkey")]
async fn reset_passkey(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let username = get_name_in_token(&req)?;
    user_model::update_passkey_by_username(&client, &username, &generate_passkey(&username)?)
        .await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[post("/transfer_money")]
async fn transfer_money(
    data: web::Json<TransferRequest>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    let username = claim.sub;
    if is_not_ordinary_user(claim.role) {
        return Err(Error::NoPermission);
    }

    user_info_model::transfer_money_by_name(&client, &username, &data.to, data.amount).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[get("/send_activation")]
async fn send_activation(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let id = deserialize_from_req!(req, IdWrapper).id;
    let user = user_model::find_user_by_id(&client, id).await?;

    let code = generate_random_code();
    activation_model::update_or_add_activation(&client, id, &code).await?;

    let mut body = get_from_config_cf_untyped!("ACTIVATE_EMAIL");
    body.push_str(&format!("?id={}&code={}", id, code));
    std::thread::spawn(move || {
        send_mail(&user.username, &user.email, "SOPT", body, "ACTIVATE")
            .expect("unable to send mail");
    });

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[get("/activate")]
async fn activate(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let data = deserialize_from_req!(req, ActivateRequest);
    let validation = activation_model::find_activation_by_id(&client, data.id).await?;
    if data.code != validation.code {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("activation code invalid")));
    }
    if validation.used {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("already activated")));
    }

    activation_model::update_activated_by_id(&client, data.id).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[get("/forget_password")]
async fn forget_password(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let email = deserialize_from_req!(req, EmailWrapper).email;
    let user = user_model::find_user_by_email(&client, &email).await?;
    let code = generate_random_code();
    let original_code =
        ROCKSDB.get_cf(ROCKSDB.cf_handle("reset").unwrap(), &user.id.to_ne_bytes())?;
    if original_code.is_none() {
        put_cf("reset", &user.id.to_ne_bytes(), &code)?;
    } else {
        let original_code = String::from_utf8(original_code.unwrap()).map_err(error_string)?;
        let time = get_time_from_code(original_code)?;
        if chrono::Utc::now().timestamp() - time < 60 {
            return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("retry too often")));
        } else {
            put_cf("reset", &user.id.to_ne_bytes(), &code)?;
        }
    }

    let mut body = get_from_config_cf_untyped!("PASSWORD_RESET_EMAIL");
    body.push_str(&format!("?id={}&code={}", user.id, code));
    std::thread::spawn(move || {
        send_mail(&user.username, &email, "SOPT", body, "RESET PASSWORD")
            .expect("unable to send mail");
    });

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[post("/validate_reset")]
async fn validate_reset(
    data: web::Json<PasswordWrapper>,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    if data.id.is_none() || data.code.is_none() {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("missing fields")));
    }
    let id = data.id.unwrap();

    let code = ROCKSDB.get_cf(ROCKSDB.cf_handle("reset").unwrap(), &id.to_ne_bytes())?;
    if code.is_none() {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("request code doesn't exist")));
    } else {
        let code = String::from_utf8(code.unwrap()).map_err(error_string)?;
        if code != data.code.as_deref().unwrap() {
            return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("invalid code")));
        }

        let time = get_time_from_code(code)?;
        if chrono::Utc::now().timestamp() - time > 1800 {
            return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("code has expired")));
        }
        ROCKSDB.delete_cf(ROCKSDB.cf_handle("reset").unwrap(), &id.to_ne_bytes())?;
    }

    let name = user_model::find_user_by_id(&client, id).await?.username;
    user_model::update_password_by_username(&client, &name, &hash_password(&data.password)?)
        .await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

pub(crate) fn user_service() -> Scope {
    web::scope("/user")
        .service(add_user)
        .service(login)
        .service(personal_info_update)
        .service(upload_avatar)
        .service(show_user)
        .service(show_torrent_status)
        .service(
            web::scope("/auth")
                .service(reset_password)
                .service(reset_passkey)
                .service(transfer_money)
                .service(send_activation)
                .service(activate)
                .service(forget_password)
                .service(validate_reset),
        )
}
