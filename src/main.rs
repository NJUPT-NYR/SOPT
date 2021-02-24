mod config;
mod data;
mod error;
mod controller;
mod util;

use crate::config::Config;
use actix_web::{HttpServer, App, middleware};
use actix_identity::{IdentityService, CookieIdentityPolicy};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use rand::Rng;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv().ok();
    println!("==========SOPT is running==========");

    let cfg = Config::from_env().unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(cfg.pg.max_connections)
        .connect(&cfg.pg.to_address())
        .await
        .expect("unable to connect to database, aborting...");
    let redis_pool = cfg.redis.create_pool().unwrap();
    let cookie_key: [u8; 32] = rand::thread_rng().gen();

    HttpServer::new(move ||
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&cookie_key)
                    .name("user-auth")
                    // Allow http?
                    .secure(false)
            ))
            .data(pool.clone())
            .data(redis_pool.clone())
            .service(controller::api_service())
    ).workers(4)
        .bind(cfg.server_addr)?
        .run().await
}
