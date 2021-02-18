mod config;
mod data;
mod error;
mod controller;

use crate::config::Config;
use actix_web::{HttpServer, App, middleware};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    dotenv().ok();
    env_logger::init();

    let cfg = Config::from_env().unwrap();
    // TODO: gate for tls
    let pool = cfg.pg.create_pool(tokio_postgres::NoTls).unwrap();

    println!("==========SOPT is running==========");
    HttpServer::new(move ||
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .service(controller::api_service())
    ).workers(4)
        .bind(cfg.server_addr)?
        .run().await
}
