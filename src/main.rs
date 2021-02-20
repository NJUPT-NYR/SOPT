mod config;
mod data;
mod error;
mod controller;
mod util;

use crate::config::Config;
use actix_web::{HttpServer, App, middleware};
use actix_identity::{IdentityService, CookieIdentityPolicy};
use dotenv::dotenv;
use rand::Rng;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    dotenv().ok();
    env_logger::init();
    println!("==========SOPT is running==========");

    let cfg = Config::from_env().unwrap();
    // TODO: gate for tls
    let pool = cfg.pg.create_pool(tokio_postgres::NoTls).unwrap();
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
            .service(controller::api_service())
    ).workers(4)
        .bind(cfg.server_addr)?
        .run().await
}
