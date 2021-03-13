use super::*;
use crate::data::{torrent_info as torrent_info_model,
                  tag as tag_model};

#[derive(Debug, Deserialize)]
struct TorrentList {
    pub ids: Vec<i64>,
}

fn is_no_permission_to_torrents(role: i64) -> bool {
    role & (1 << 62) == 0
}

/// list all invisible torrents
#[get("/show_invisible_torrents")]
async fn show_invisible_torrents(
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    if is_no_permission_to_torrents(claim.role) {
        return Err(Error::NoPermission)
    }
    let ret = torrent_info_model::find_invisible_torrent(&client).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

/// make a group of torrents visible
#[post("/accept_torrents")]
async fn accept_torrents(
    data: web::Json<TorrentList>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    if is_no_permission_to_torrents(claim.role) {
        return Err(Error::NoPermission)
    }
    let torrents: Vec<i64> = data.into_inner().ids;
    let ret = torrent_info_model::make_torrent_visible(&client, torrents).await?;

    for torrent in ret {
        for tag in torrent.tag.unwrap_or_default() {
            tag_model::update_or_add_tag(&client, &tag).await?;
        }
    }
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

/// make a group of torrents stick
#[post("/stick_torrents")]
async fn stick_torrents(
    data: web::Json<TorrentList>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    if is_no_permission_to_torrents(claim.role) {
        return Err(Error::NoPermission)
    }
    let torrents: Vec<i64> = data.into_inner().ids;
    torrent_info_model::make_torrent_stick(&client, torrents).await?;

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

pub fn admin_service() -> Scope {
    web::scope("/admin")
        .service(web::scope("/torrent")
            .service(accept_torrents)
            .service(stick_torrents)
            .service(show_invisible_torrents))
}