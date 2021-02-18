use crate::data::user as user_model;
use crate::error::Error;
use actix_web::*;
use deadpool_postgres::{Client, Pool};
use serde::Deserialize;
use crate::controller::HttpResult;
use crate::util::{parse_email, generate_passkey};
use crate::data::user::User;

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
pub struct Registry {
    pub email: String,
    pub username: String,
    pub password: String,
    #[cfg(invitation = "on")]
    pub invite_code: Option<String>,
}

// TODO: more elegance on validating
// TODO: like split into different pages
#[post("/add_user")]
async fn add_user(
    data: web::Json<Registry>,
    db_pool: web::Data<Pool>,
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
                                        User::new(
                                            user.email,
                                            user.username,
                                            user.password,
                                            passkey,
                                        )).await?;
    Ok(HttpResponse::Ok().json(&new_user))
}

/// Here comes danger action, so a validation must be performed.
/// By store the identity in redis, we are able to allow user
/// to perform other operations like a charm.
async fn check_identity(
    data: web::Json<Validation>,
    db_pool: web::Data<Pool>,
) -> HttpResult {
    let password = data.into_inner();
    let client: Client = db_pool.get().await.map_err(Error::PoolError)?;

    todo!()
}

async fn reset_password(
    data: web::Json<Validation>,
    db_pool: web::Data<Pool>,
) -> HttpResult {
    let password = data.into_inner();
    let client: Client = db_pool.get().await.map_err(Error::PoolError)?;

    todo!()
}

async fn reset_passkey(
    db_pool: web::Data<Pool>,
) -> HttpResult {
    let client: Client = db_pool.get().await.map_err(Error::PoolError)?;

    todo!()
}

pub fn user_service() -> Scope {
    web::scope("/user")
        .service(add_user)
}