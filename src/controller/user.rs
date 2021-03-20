use super::*;
use crate::data::{user as user_model,
                  invitation as invitation_model,
                  user_info as user_info_model,
                  rank as rank_model};

static ALLOWED_AVATAR_EXTENSION: [&str; 4] = [
    "jpg",
    "jpeg",
    "png",
    "webp",
];

#[derive(Deserialize, Debug)]
struct Registry {
    email: String,
    username: String,
    password: String,
    invite_code: Option<String>,
}

/// sign up controller
#[post("/add_user")]
async fn add_user(
    data: web::Json<Registry>,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let user = data.into_inner();
    let mut allowed = false;
    let mut code = None;

    match parse_email(&user.email) {
        Some(_email) => {
            if let None = user.invite_code {
                #[cfg(feature = "email-restriction")]
                if ALLOWED_DOMAIN.read().unwrap().get(&_email.domain).is_some() {
                    allowed = true;
                }
            }
        },
        None => return Ok(HttpResponse::BadRequest().json(GeneralResponse::from_err("invalid email address"))),
    }
    if let Some(str) = user.invite_code {
        let mut ret = invitation_model::find_invitation_by_code(&client, &str).await?;
        if !ret.is_empty() {
            if ret[0].is_used {
                return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("invitation code already taken")))
            }
            code = Some(ret.pop().unwrap());
            allowed = true;
        }
    }
    if !allowed {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("not allowed to register")))
    }
    let check = user_model::check_existence(&client, &user.email, &user.username).await?;
    if !check.is_empty() {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err(&format!("{} already taken", check))))
    }

    let passkey = generate_passkey(&user.username)?;
    let new_user = user_model::add_user(&client, &user.email, &user.username, &hash_password(&user.password)?, &passkey).await?;
    user_info_model::add_user_info(&client, new_user.id, &new_user.username).await?;
    if code.is_some() {
        let true_code = code.unwrap();
        user_info_model::add_invitor_by_name(&client, &new_user.username, true_code.sender).await?;
        invitation_model::update_invitation_usage(&client, &true_code.code).await?;
    }
    Ok(HttpResponse::Ok().json(new_user.to_json()))
}

#[derive(Deserialize, Debug)]
struct Login {
    username: String,
    password: String,
}

/// use `username` and `password` to login
#[post("/login")]
async fn login(
    data: web::Json<Login>,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    use chrono::{Utc, Duration};
    use jsonwebtoken::{encode, EncodingKey, Header};

    let user = data.into_inner();
    let secret = CONFIG.secret_key.as_bytes();

    let validation = user_model::find_user_by_username(&client, &user.username).await?.pop();
    match validation {
        Some(mut val) => {
            if !verify_password(&user.password, &val.password)? {
                return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("password not match")))
            }
            user_info_model::update_activity_by_name(&client, &user.username).await?;
            let current_rank = rank_model::find_rank_by_username(&client, &user.username).await?;
            if current_rank.next.is_some() {
                let next_rank = rank_model::find_rank_by_id(&client, current_rank.next.unwrap()).await?;
                let info = user_info_model::find_user_info_by_name(&client, &user.username).await?;
                let current = Utc::now().timestamp();
                let before = info.register_time.timestamp();
                if info.upload > next_rank.upload && current - before > next_rank.age {
                    let roles = next_rank.role;
                    for role in roles {
                        user_model::add_role_by_id(&client, val.id, (role % 32) as i32).await?;
                    }
                    user_info_model::update_rank_by_name(&client, &user.username, &next_rank.name).await?;
                    val = user_model::find_user_by_username(&client, &user.username).await?.pop().unwrap();
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
                &EncodingKey::from_secret(secret)).unwrap();
            Ok(HttpResponse::Ok().json(tokens.to_json()))
        },
        None => Ok(HttpResponse::Ok().json(GeneralResponse::from_err("password not match"))),
    }
}

#[derive(Deserialize, Debug)]
struct InfoWrapper {
    info: serde_json::Value,
}

/// update user defined json fields
/// replace all without any check
#[post("/personal_info_update")]
async fn personal_info_update(
    data: web::Json<InfoWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let username = get_name_in_token(req)?;
    user_info_model::update_other_by_name(&client, &username, &data.info).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

/// upload avatar and b64encode it into database
#[post("/upload_avatar")]
async fn upload_avatar(
    mut payload: actix_multipart::Multipart,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    use futures::{StreamExt, TryStreamExt};

    let username = get_name_in_token(req)?;

    if let Ok(Some(mut file)) = payload.try_next().await {
        let content_type = file.content_disposition().ok_or(Error::OtherError("incomplete file".to_string()))?;
        let filename = content_type.get_filename().ok_or("incomplete file".to_string())?;
        let cleaned_name = sanitize_filename::sanitize(&filename);

        let suffix = cleaned_name.rfind('.').ok_or(Error::OtherError("missing filename extension".to_string()))?;
        let ext = cleaned_name[suffix+1..].to_ascii_lowercase();
        if ALLOWED_AVATAR_EXTENSION.iter().find(|x| x == &&ext.as_str()).is_none() {
            return Ok(HttpResponse::UnsupportedMediaType().json(GeneralResponse::from_err("must be jpg or png or webp")))
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

#[derive(Deserialize, Debug)]
struct PrivacyLevel {
    privacy: i32,
}

/// change privacy level, by default it is 0
/// which means everyone can see your profile
#[get("change_privacy")]
async fn change_privacy(
    web::Query(data): web::Query<PrivacyLevel>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    use std::convert::TryInto;

    let username = get_name_in_token(req)?;
    let level: user_info_model::Level = data.privacy.try_into()?;

    user_info_model::update_privacy_by_name(&client, &username, level).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[derive(Deserialize, Debug)]
struct UserRequest {
    username: String,
}

/// show detail of one user
/// if login, you can see your account info too
#[get("show_user")]
async fn show_user(
    web::Query(data): web::Query<UserRequest>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    let username = claim.sub;

    let info = user_info_model::find_user_info_by_name(&client, &data.username).await?;
    if username.eq(&data.username) {
        let account = user_model::find_user_by_username_slim(&client, &username).await?;
        let ret = user_info_model::JoinedUser {
            info,
            account: Some(account),
        };
        Ok(HttpResponse::Ok().json(ret.to_json()))
    } else {
        if info.privacy > 0 && is_no_permission_to_users(claim.role) {
            Err(Error::NoPermission)
        } else {
            let ret = user_info_model::JoinedUser {
                info,
                account: None,
            };
            Ok(HttpResponse::Ok().json(ret.to_json()))
        }
    }
}

#[derive(Deserialize, Debug)]
struct PassWrapper {
    password: String,
}

/// reset user password
#[post("/reset_password")]
async fn reset_password(
    data: web::Json<PassWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let username = get_name_in_token(req)?;
    user_model::update_password_by_username(&client, &username, &hash_password(&data.password)?).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

/// reset user passkey
#[get("/reset_passkey")]
async fn reset_passkey(
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let username = get_name_in_token(req)?;
    user_model::update_passkey_by_username(&client, &username, &generate_passkey(&username)?).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[derive(Deserialize, Debug)]
struct Transfer {
    to: String,
    amount: f64,
}

/// transfer certain money to user
#[post("/transfer_money")]
async fn transfer_money(
    data: web::Json<Transfer>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    let username = claim.sub;
    if is_not_ordinary_user(claim.role) {
        return Err(Error::NoPermission)
    }

    user_info_model::transfer_money_by_name(&client, &username, &data.to, data.amount).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

pub(crate) fn user_service() -> Scope {
    web::scope("/user")
        .service(add_user)
        .service(login)
        .service(personal_info_update)
        .service(upload_avatar)
        .service(change_privacy)
        .service(show_user)
        .service(web::scope("/auth")
            .service(reset_password)
            .service(reset_passkey)
            .service(transfer_money))
}
