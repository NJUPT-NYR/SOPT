mod config;
mod controller;
mod data;
mod error;
mod util;

use crate::config::Config;
use actix_web::{middleware, web::route, App, HttpResponse, HttpServer};
use dotenv::dotenv;

/// a key wrapper is used in case of
/// actix data mechanism
#[derive(Clone, Debug)]
pub(crate) struct KeyWrapper(Box<String>);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv().ok();

    let cfg = Config::from_env().unwrap();
    let pool = sqlx::PgPool::connect(&cfg.database_url)
        .await
        .expect("unable to connect to database");
    let key = KeyWrapper(Box::new(cfg.secret_key));
    println!("==========SOPT is running==========");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .data(key.clone())
            .service(controller::api_service())
            .default_service(route().to(|| HttpResponse::NotFound().body("Not Found")))
    })
    .workers(4)
    .bind(cfg.server_addr)?
    .run()
    .await
}
