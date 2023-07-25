mod config;
mod error;
mod filter;
mod tracker_route;

use crate::config::CONFIG;
use actix_web::*;
use tracker_route::*;

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    log4rs::init_file("config/log4rs_tracker.yaml", Default::default()).unwrap();
    println!("⭐⭐⭐⭐⭐⭐⭐⭐⭐ Initializing filter ⭐⭐⭐⭐⭐⭐⭐⭐⭐");
    let keys = get_passkey_from_db().await;
    context::CONTEXT.filter.expand(keys).await;
    println!("⭐⭐⭐⭐⭐⭐⭐⭐ SOPT tracker is running ⭐⭐⭐⭐⭐⭐⭐⭐");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::new("%a \"%r\" %s %T"))
            .service(tracker_service())
            .default_service(
                web::route().to(|| async { HttpResponse::NotFound().body("Not Found") }),
            )
    })
    .bind(&CONFIG.tracker_addr)?
    .run()
    .await
}

fn main() {
    start_server().expect("GG");
}
