use super::*;
use crate::data::{ToResponse, GeneralResponse, Claim,
                  user as user_model,
                  invitation as invitation_model,
                 user_info as user_info_model};

/// Allow email to register
#[cfg(feature = "email-restriction")]
static ALLOWED_DOMAIN: [&str; 3] = [
    "gmail.com",
    "njupt.edu.cn",
    "outlook.com"
];

static ALLOWED_AVATAR_EXTENSION: [&str; 4] = [
    "jpg",
    "jpeg",
    "png",
    "webp",
];

#[derive(Deserialize, Debug)]
struct Validation {
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Registry {
    pub email: String,
    pub username: String,
    pub password: String,
    pub invite_code: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
struct Transfer {
    pub to: String,
    pub amount: f64,
}

#[derive(Deserialize, Debug)]
struct InfoWrapper {
    pub info: serde_json::Value,
}

/// sign up controller
#[post("/add_user")]
async fn add_user(
    data: web::Json<Registry>,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let user: Registry = data.into_inner();
    // not so elegant
    let mut allowed = false;
    let mut code: Option<invitation_model::InvitationCode> = None;

    match parse_email(user.email.as_str()) {
        Some(_email) => {
            if let None = user.invite_code {
                #[cfg(feature = "email-restriction")]
                if ALLOWED_DOMAIN.iter().find(|x| x == &&_email.domain.as_str()).is_some() {
                    allowed = true;
                }
            }
        },
        None => return Ok(HttpResponse::BadRequest().json(GeneralResponse::from_err("invalid email address"))),
    };

    if let Some(str) = user.invite_code {
        let mut ret = invitation_model::find_invitation_by_code(&client, &str).await?;
        if !ret.is_empty() {
            let is_used = ret.first().unwrap().is_used;
            if is_used {
                return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("invitation code already be used")))
            }
            code = Some(ret.pop().unwrap());
            allowed = true;
        }
    }

    if !allowed {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("not allowed to register")))
    }

    if !user_model::find_user_by_username(&client, &user.username)
        .await?
        .is_empty() {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("username already taken")))
    } else if !user_model::find_user_by_email(&client, &user.email)
        .await?
        .is_empty() {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("email already registered")))
    }

    let passkey = generate_passkey(&user.username)?;
    let new_user = user_model::add_user(
        &client,
        user_model::User::new(
            user.email,
            user.username,
            hash_password(&user.password)?,
            passkey,
        )).await?;
    user_info_model::add_user_info(&client, new_user.id, &new_user.username).await?;
    if code.is_some() {
        let true_code = code.unwrap();
        user_info_model::add_invitor_by_name(&client, &new_user.username, true_code.sender).await?;
        invitation_model::update_invitation_usage(&client, &true_code.code).await?;
    }
    Ok(HttpResponse::Ok().json(new_user.to_json()))
}

/// use `username` and `password` to login
#[post("/login")]
async fn login(
    data: web::Json<Login>,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    use chrono::{Utc, Duration};
    use jsonwebtoken::{encode, EncodingKey, Header};

    let user: Login = data.into_inner();
    let secret: &[u8] = CONFIG.secret_key.as_bytes();

    let validation = user_model::find_user_by_username(&client, &user.username)
        .await?.pop();
    match validation {
        Some(val) => {
            if !verify_password(&user.password, &val.password)? {
                return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("password not match")))
            }
            user_info_model::update_activity_by_name(&client, &user.username).await?;
            let claim = Claim {
                sub: user.username,
                exp: (Utc::now() + Duration::days(30)).timestamp() as u64,
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

/// update user defined json fields
/// replace all without any check
#[post("/personal_info_update")]
async fn personal_info_update(
    data: web::Json<InfoWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let data: InfoWrapper = data.into_inner();
    let username = get_name_in_token(req)?;

    let ret = user_info_model::update_other_by_name(&client, &username, data.info).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
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
            let data = chunk.unwrap();
            buf.append(&mut data.to_vec());
        }
        let encoded_avatar = base64::encode(buf);
        user_info_model::update_avatar_by_name(&client, &username, encoded_avatar).await?;
    }

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

/// Here comes danger action, so a validation must be performed.
#[post("/check_identity")]
async fn check_identity(
    data: web::Json<Validation>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let password: String = data.into_inner().password;
    let username = get_name_in_token(req)?;

    let validation = user_model::find_user_by_username(&client, &username)
        .await?.pop().expect("Someone hacked our token!");

    if verify_password(&password, &validation.password)? {
        Ok(HttpResponse::Ok().json(GeneralResponse::default()))
    } else {
        Ok(HttpResponse::Ok().json(GeneralResponse::from_err("password not match")))
    }
}

/// reset user password
#[post("/reset_password")]
async fn reset_password(
    data: web::Json<Validation>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let new_pass = hash_password(&data.into_inner().password)?;
    let username = get_name_in_token(req)?;

    user_model::update_password_by_username(&client, &username, &new_pass).await?;
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

/// transfer certain money to user
#[post("/transfer_money")]
async fn transfer_money(
    data: web::Json<Transfer>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let data: Transfer = data.into_inner();
    let username = get_name_in_token(req)?;

    user_info_model::transfer_money_by_name(&client, &username, &data.to, data.amount).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

pub fn user_service() -> Scope {
    web::scope("/user")
        .service(add_user)
        .service(login)
        .service(personal_info_update)
        .service(upload_avatar)
        .service(web::scope("/auth")
            .service(check_identity)
            .service(reset_password)
            .service(reset_passkey)
            .service(transfer_money))
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use serde_json::*;
    use super::*;

    #[actix_rt::test]
    async fn test_signup_and_login() {
        dotenv().ok();
        let pool = sqlx::PgPool::connect(&CONFIG.database_url)
            .await
            .expect("unable to connect to database");
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .service(crate::controller::api_service()))
            .await;

        let signup_ok = json!({
            "email": "tadokoro@gmail.com",
            "username": "kusa114",
            "password": "114514",
        });
        let req = test::TestRequest::post().uri("/api/user/add_user").set_json(&signup_ok).to_request();
        let resp: GeneralResponse = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.success, true);
        // duplicate register should fail
        let req = test::TestRequest::post().uri("/api/user/add_user").set_json(&signup_ok).to_request();
        let resp: GeneralResponse = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.success, false);

        let login_ok  = json!({
            "username": "kusa114",
            "password": "114514",
        });
        let req = test::TestRequest::post().uri("/api/user/login").set_json(&login_ok).to_request();
        let resp: GeneralResponse = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.success, true);

        let login_failed_1 = json!({
            "username": "kusa114",
            "password": "1919810",
        });
        let req = test::TestRequest::post().uri("/api/user/login").set_json(&login_failed_1).to_request();
        let resp: GeneralResponse = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.success, false);

        let login_failed_2 = json!({
            "username": "phantom_user",
            "password": "fake_@pass",
        });
        let req = test::TestRequest::post().uri("/api/user/login").set_json(&login_failed_2).to_request();
        let resp: GeneralResponse = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.success, false);

        #[cfg(feature = "email-restriction")]
        {
            let signup_banned_mail = json!({
                "email": "thief@banned.org",
                "username": "anyway",
                "password": "114514",
            });
            let req = test::TestRequest::post().uri("/api/user/add_user").set_json(&signup_banned_mail).to_request();
            let resp: GeneralResponse = test::read_response_json(&mut app, req).await;
            assert_eq!(resp.success, false);
        }

        // not pollute database
        sqlx::query!("DELETE FROM user_info WHERE username = 'kusa114';").execute(&pool).await.unwrap();
        sqlx::query!("DELETE FROM users WHERE username = 'kusa114';").execute(&pool).await.unwrap();
    }

    #[actix_rt::test]
    async fn test_personal_info_update() {
        use crate::data::user::User;

        dotenv().ok();
        let pool = sqlx::PgPool::connect(&CONFIG.database_url)
            .await
            .expect("unable to connect to database");
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .service(crate::controller::api_service()))
            .await;

        let ret = crate::data::user::add_user(&pool, User::new(
            "yuki@nago.to".to_string(),
            "YUKI.N".to_string(),
            crate::util::hash_password("20060707").unwrap(),
            "akdaskjkaschakjsc".to_string()
        )).await.unwrap();
        crate::data::user_info::add_user_info(&pool, ret.id, &ret.username).await.unwrap();

        let login_ok  = json!({
            "username": "YUKI.N",
            "password": "20060707",
        });
        let req = test::TestRequest::post().uri("/api/user/login").set_json(&login_ok).to_request();
        let resp: GeneralResponse = test::read_response_json(&mut app, req).await;
        let token: String = from_value(resp.data).unwrap();

        let data = json!({
            "info": json!({
                "高校": "县立北高",
                "Job": "信息统合生命体对有机生命体接触用人形终端",
                "紹介": "まだ図書館に行く"
            })
        });
        let req = test::TestRequest::post().set_json(&data)
            .header("Authorization", format!("Bearer {}", token))
            .uri("/api/user/personal_info_update").to_request();
        let resp: GeneralResponse = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.success, true);

        sqlx::query!("DELETE FROM user_info WHERE username = 'YUKI.N';").execute(&pool).await.unwrap();
        sqlx::query!("DELETE FROM users WHERE username = 'YUKI.N';").execute(&pool).await.unwrap();
    }
}
