mod config;
mod controller;
pub mod data;
mod error;
mod search;
mod util;

use crate::config::*;
use actix_web::{middleware, web::route, App, HttpResponse, HttpServer};
use dotenv::dotenv;

/// load email whitelist from file `filtered-email`
async fn load_email_whitelist() {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open("./config/filtered-email").expect("email allowlist not exist");
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut w = controller::ALLOWED_DOMAIN.write().await;
    *w = HashSet::from_iter(lines);
}

/// initializing searching engine from database
async fn initializing_search(client: &sqlx::PgPool) {
    let rets = sqlx::query!("SELECT id, title, poster, tag FROM torrent_info;")
        .fetch_all(client)
        .await
        .unwrap();
    let mut w = search::TORRENT_SEARCH_ENGINE.write().await;
    for ret in rets {
        let mut tokens = vec![ret.title, ret.poster];
        tokens.append(&mut ret.tag.unwrap_or_default());
        w.insert(ret.id, tokens);
    }
}

/// insert settings into persistent KV storage
/// may overwrite past settings if restart
fn init_settings() {
    use crate::data::kv::KVDB;

    for (key, val) in controller::STRING_SITE_SETTING.iter() {
        KVDB.clone()
            .put("config", key.as_ref(), val.as_ref())
            .unwrap();
    }
    // TODO: Reflection?
    KVDB.clone()
        .put("config", "INVITE CONSUME".as_ref(), &5000_f64.to_ne_bytes())
        .unwrap();
    KVDB.clone()
        .put(
            "config",
            "BAN UPLOAD RATIO".as_ref(),
            &0.3_f64.to_ne_bytes(),
        )
        .unwrap();
    KVDB.clone()
        .put("config", "NEWBIE TERM".as_ref(), &14_i64.to_ne_bytes())
        .unwrap();
    KVDB.clone()
        .put("config", "LOGIN EXPIRE DAY".as_ref(), &3_i64.to_ne_bytes())
        .unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    dotenv().ok();
    println!("⭐⭐⭐⭐⭐⭐⭐⭐⭐Initializing configurations⭐⭐⭐⭐⭐⭐⭐⭐⭐");
    load_email_whitelist().await;
    init_settings();
    println!("⭐⭐⭐⭐⭐⭐⭐⭐⭐Initializing search engines⭐⭐⭐⭐⭐⭐⭐⭐⭐");
    let pool = sqlx::PgPool::connect(&CONFIG.database_url)
        .await
        .expect("unable to connect to database");
    initializing_search(&pool).await;
    println!("⭐⭐⭐⭐⭐⭐⭐⭐⭐SOPT is running⭐⭐⭐⭐⭐⭐⭐⭐⭐");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::new("%a \"%r\" %s %T"))
            .app_data(pool.clone())
            .service(controller::api_service())
            .default_service(route().to(|| async { HttpResponse::NotFound().body("Not Found") }))
    })
    .workers(4)
    .bind(&CONFIG.server_addr)?
    .run()
    .await
}
