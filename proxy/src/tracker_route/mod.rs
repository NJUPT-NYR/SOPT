pub(crate) mod context;
mod data;

use crate::config::CONFIG;
use crate::error::ProxyError;
use actix_web::*;
use bendy::encoding::ToBencode;
use context::CONTEXT;
use data::{AnnounceBypassData, AnnounceRequestData, AnnounceResponseData, UpdateFilterCommand};
use deadpool_redis::redis::Value;

type ProxyResult = Result<HttpResponse, ProxyError>;

pub async fn get_passkey_from_db() -> Vec<String> {
    let client = sqlx::PgPool::connect(&CONFIG.database_url)
        .await
        .expect("unable to connect to database");

    let rets: Vec<String> = sqlx::query!("SELECT passkey FROM users WHERE role & (1::BIGINT) = 1;")
        .fetch_all(&client)
        .await
        .unwrap()
        .into_iter()
        .map(|r| String::from(r.passkey))
        .collect();
    rets
}

#[get("/announce")]
async fn announce(
    web::Query(mut q): web::Query<AnnounceRequestData>,
    req: HttpRequest,
) -> ProxyResult {
    let peer_ip = req.peer_addr().map(|addr| addr.ip());
    CONTEXT.validation(&q).await?;
    q.fix_ip(peer_ip);

    let mut cxn = CONTEXT.pool.get().await?;
    let cmd = q.generate_announce_cmd();
    let t: Vec<Value> = cmd.query_async(&mut cxn).await?;
    let response = AnnounceResponseData::from(t);
    let x = response.to_bencode()?;

    let req = AnnounceBypassData::from(q);
    let req = serde_qs::to_string(&req)?;
    let addr = format!(
        "http://{}/api/tracker/get_announce?{}",
        CONFIG.server_addr, req
    );
    let resp = reqwest::get(&addr)
        .await
        .map_err(|_| ProxyError::RequestError("bypass to backend failed"))?;
    if !resp.status().is_success() {
        return Err(ProxyError::RequestError("bypass to backend failed"));
    }

    Ok(HttpResponse::Ok().body(x))
}

#[post("update_filter")]
async fn update_filter(query: web::Json<UpdateFilterCommand>) -> ProxyResult {
    let query = query.into_inner();
    if query.delete.is_some() {
        CONTEXT.filter.delete(query.delete.unwrap()).await;
    }
    if query.set.is_some() {
        CONTEXT.filter.insert(query.set.unwrap()).await;
    }

    tokio::spawn(async move {
        if CONTEXT.filter.check_expand() {
            let keys = get_passkey_from_db().await;
            CONTEXT.filter.expand(keys).await;
        }
    });

    Ok(HttpResponse::Ok().finish())
}

pub fn tracker_service() -> Scope {
    web::scope("/tracker")
        .service(announce)
        .service(update_filter)
}
