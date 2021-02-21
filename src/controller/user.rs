use actix_web::*;
use actix_identity::Identity;
use deadpool_postgres::Client;
use deadpool_redis::Pipeline;
use serde::Deserialize;
use super::*;
use crate::util::*;
use crate::data::user as user_model;
use crate::error::Error;

#[cfg(email_restriction = "on")]
static ALLOWED_DOMAIN: [&str; 3] = [
    "gmail.com",
    "njupt.edu.cn",
    "outlook.com"
];

#[derive(Deserialize, Debug)]
struct Validation {
    pub password: String,
}

#[derive(Deserialize, Debug)]
struct Registry {
    pub email: String,
    pub username: String,
    pub password: String,
    #[cfg(invitation = "on")]
    pub invite_code: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Login {
    pub username: String,
    pub password: String,
}

// TODO: more elegance on validating
// TODO: like split into different pages
#[post("/add_user")]
async fn add_user(
    data: web::Json<Registry>,
    db_pool: web::Data<deadpool_postgres::Pool>,
) -> HttpResult {
    let user = data.into_inner();
    let client: Client = db_pool.get().await.map_err(Error::PoolError)?;

    // TODO: add user via invite code

    match parse_email(user.email.as_str()) {
        Some(_email) => {
            #[cfg(email_restriction = "on")]
            if ALLOWED_DOMAIN.iter().find(|x| x == &&_email.domain.as_str()).is_none() {
                return Ok(HttpResponse::Ok().body("email address not allowed"))
            }
        },
        None => return Ok(HttpResponse::Ok().body("invalid email address"))
    };

    if !user_model::find_user_by_username(&client, &user.username)
        .await?
        .is_empty() {
        return Ok(HttpResponse::Ok().body("username already taken"))
    } else if !user_model::find_user_by_email(&client, &user.email)
        .await?
        .is_empty() {
        return Ok(HttpResponse::Ok().body("email already registered"))
    }

    let passkey = generate_passkey(&user.username);
    let new_user = user_model::add_user(&client,
                                        user_model::User::new(
                                            user.email,
                                            user.username,
                                            hash_password(&user.password),
                                            passkey,
                                        )).await?;
    Ok(HttpResponse::Ok().json(&new_user))
}

#[post("/login")]
async fn login(
    data: web::Json<Login>,
    id: Identity,
    db_pool: web::Data<deadpool_postgres::Pool>,
) -> HttpResult {
    let user = data.into_inner();
    let client: Client = db_pool.get().await.map_err(Error::PoolError)?;

    let validation = user_model::find_user_by_username_full(&client, &user.username)
        .await?.pop();
    match validation {
        Some(val) => {
            if !verify_password(&user.password, &val.password.unwrap()) {
                return Ok(HttpResponse::Ok().body("wrong password"))
            }
            id.remember(user.username);
            Ok(HttpResponse::Ok().finish())
        },
        None => Ok(HttpResponse::Ok().body("user not registered")),
    }
}

#[get("/logout")]
async fn logout(id: Identity) -> HttpResult {
    id.forget();
    Ok(HttpResponse::Ok().finish())
}

/// Here comes danger action, so a validation must be performed.
/// By store the identity in redis, we are able to allow user
/// to perform other operations like a charm.
/// TODO: VERY AD-HOC FOR NOW
#[post("/check_identity")]
async fn check_identity(
    data: web::Json<Validation>,
    id: Identity,
    db_pool: web::Data<deadpool_postgres::Pool>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResult {
    let password = data.into_inner().password;
    let client: Client = db_pool.get().await.map_err(Error::PoolError)?;
    let username = id.identity().ok_or(Error::CookieError)?;
    let mut conn = redis_pool.get().await.map_err(Error::RedisError)?;

    let validation =
        user_model::find_user_by_username_full(&client, &username)
        .await?.pop().unwrap();

    if verify_password(&password, &validation.password.unwrap()) {
        let mut pipe = Pipeline::new();
        pipe.set_ex(&username, "authed", 300);
        pipe.execute_async(&mut conn).await.unwrap();
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Ok().body("wrong password"))
    }
}

#[post("/reset_password")]
async fn reset_password(
    data: web::Json<Validation>,
    id: Identity,
    db_pool: web::Data<deadpool_postgres::Pool>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResult {
    let new_pass = hash_password(&data.into_inner().password);
    let client: Client = db_pool.get().await.map_err(Error::PoolError)?;
    let username = id.identity().ok_or(Error::CookieError)?;
    let mut conn = redis_pool.get().await.map_err(Error::RedisError)?;

    let mut pipe = Pipeline::new();
    pipe.get(&username);
    let resp: String = pipe.query_async(&mut conn).await.unwrap();
    if resp == "nil" {
        return Ok(HttpResponse::Ok().body("not authed yet"));
    }

    let ret_user =
        user_model::update_password_by_username(&client, &username, &new_pass)
            .await?;
    Ok(HttpResponse::Ok().json(&ret_user))
}

#[get("/reset_passkey")]
async fn reset_passkey(
    id: Identity,
    db_pool: web::Data<deadpool_postgres::Pool>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResult {
    let client: Client = db_pool.get().await.map_err(Error::PoolError)?;
    let username = id.identity().ok_or(Error::CookieError)?;
    let mut conn = redis_pool.get().await.map_err(Error::RedisError)?;

    let mut pipe = Pipeline::new();
    pipe.get(&username);
    let resp: String = pipe.query_async(&mut conn).await.unwrap();
    if resp == "nil" {
        return Ok(HttpResponse::Ok().body("not authed yet"));
    }

    let ret_user =
        user_model::update_passkey_by_username(&client, &username, &generate_passkey(&username))
            .await?;
    Ok(HttpResponse::Ok().json(&ret_user))
}

pub fn user_service() -> Scope {
    web::scope("/user")
        .service(add_user)
        .service(login)
        .service(logout)
        .service(web::scope("/auth")
            .service(check_identity)
            .service(reset_password)
            .service(reset_passkey))
}