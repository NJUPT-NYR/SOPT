mod config;
mod controller;
pub mod data;
mod error;
mod util;
mod search;

use crate::config::*;
use actix_web::{middleware, web::route, App, HttpResponse, HttpServer};
use dotenv::dotenv;

/// load email whitelist from file `filtered-email`
fn load_email_whitelist() {
    use std::fs::File;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use std::io::{BufReader, BufRead};

    let file = File::open("filtered-email")
        .expect("email whitelist not exist");
    let lines: Vec<String> = BufReader::new(file).lines()
        .map(|l| String::from(l.unwrap()))
        .collect();

    let mut w = controller::ALLOWED_DOMAIN.write().unwrap();
    *w = HashSet::from_iter(lines);
}

async fn initializing_search(client: &sqlx::PgPool) {
    let rets = sqlx::query!(
        "SELECT id, title, poster, tag FROM torrent_info;"
    ).fetch_all(client).await.unwrap();
    let mut w = search::TORRENT_SEARCH_ENGINE.write().unwrap();
    for ret in rets {
        let mut tokens = vec![ret.title, ret.poster];
        tokens.append(&mut ret.tag.unwrap_or_default());
        w.insert(ret.id, tokens);
    }
}

// fn init_settings(db: &rocksdb::DB) {
//     db.put("INVITE_CONSUME", 5000).unwrap();
// }

#[actix_web::main]
pub async fn sopt_main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv().ok();
    println!("==========Initializing configurations==========");
    load_email_whitelist();
    // let rocksdb = rocksdb::DB::open_default(&CONFIG.rocksdb_path)
    //     .expect("unable to connect to rocksdb");
    // init_settings(&rocksdb);
    println!("==========Initializing search engines==========");
    let pool = sqlx::PgPool::connect(&CONFIG.database_url)
        .await
        .expect("unable to connect to database");
    initializing_search(&pool).await;
    println!("================SOPT is running================");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            // .data(rocksdb)
            .service(controller::api_service())
            .default_service(route().to(|| HttpResponse::NotFound().body("Not Found")))
    })
        .workers(4)
        .bind(&CONFIG.server_addr)?
        .run()
        .await
}
