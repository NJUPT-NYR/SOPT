mod config;
mod controller;
pub mod data;
mod error;
mod util;

use crate::config::*;
use actix_web::{middleware, web::route, App, HttpResponse, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
pub async fn sopt_main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv().ok();

    let pool = sqlx::PgPool::connect(&CONFIG.database_url)
        .await
        .expect("unable to connect to database");
    println!("==========SOPT is running==========");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .service(controller::api_service())
            .default_service(route().to(|| HttpResponse::NotFound().body("Not Found")))
    })
        .workers(4)
        .bind(&CONFIG.server_addr)?
        .run()
        .await
}
