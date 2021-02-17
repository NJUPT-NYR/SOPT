mod config;
mod data;
mod error;
mod controller;

use crate::config::Config;
use actix_web::{HttpServer, App, web};
use dotenv::dotenv;
use controller::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let cfg = Config::from_env().unwrap();
    let pool = cfg.pg.create_pool(tokio_postgres::NoTls).unwrap();

    println!("==========SOPT is running==========");
    HttpServer::new(move ||
        App::new()
            .data(pool.clone())
            .service(web::resource("/user/add_user").route(
                web::post().to(user::add_user)))
    ).bind(cfg.server_addr)?
        .run().await
}
