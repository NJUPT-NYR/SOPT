mod config;
mod controller;
mod data;
mod error;
mod util;

use crate::config::Config;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web::route, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use rand::Rng;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv().ok();

    let cfg = Config::from_env().unwrap();
    let pool = sqlx::PgPool::connect(&cfg.database_url)
        .await
        .expect("unable to connect to database");
    let redis_pool = cfg.redis.create_pool().unwrap();
    let cookie_key: [u8; 32] = rand::thread_rng().gen();
    println!("==========SOPT is running==========");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&cookie_key)
                    .name("user-auth")
                    // Allow http?
                    .secure(false),
            ))
            .data(pool.clone())
            .data(redis_pool.clone())
            .service(controller::api_service())
            .default_service(route().to(|| HttpResponse::NotFound().body("Not Found")))
    })
    .workers(4)
    .bind(cfg.server_addr)?
    .run()
    .await
}
