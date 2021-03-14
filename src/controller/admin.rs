use super::*;
use crate::data::{torrent_info as torrent_info_model,
                  tag as tag_model,
                  user as user_model,
                  user_info as user_info_model};

fn is_not_su(role: i64) -> bool {
    role & (1 << 63) == 0
}

fn is_no_permission_to_torrents(role: i64) -> bool {
    role & (1 << 62) == 0
}

fn is_no_permission_to_users(role: i64) -> bool {
    role & (1 << 61) == 0
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

#[derive(Debug, Deserialize)]
struct TorrentList {
    ids: Vec<i64>,
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
    let ret = torrent_info_model::make_torrent_visible(&client, &data.ids).await?;

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
    torrent_info_model::make_torrent_stick(&client, &data.ids).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[derive(Deserialize, Debug)]
struct IdWrapper {
    id: i64,
}

/// I hope this never get used
#[get("/ban_user")]
async fn ban_user(
    data: web::Json<IdWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    if is_no_permission_to_users(claim.role) {
        return Err(Error::NoPermission)
    }
    user_model::delete_role_by_id(&client, data.id, 0).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

/// unban one user
#[get("/unban_user")]
async fn unban_user(
    data: web::Json<IdWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    if is_no_permission_to_users(claim.role) {
        return Err(Error::NoPermission)
    }
    user_model::add_role_by_id(&client, data.id, 0).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

/// list all banned user
#[get("/list_banned_user")]
async fn list_banned_user(
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    if is_no_permission_to_users(claim.role) {
        return Err(Error::NoPermission)
    }
    let ret = user_model::list_banned_user(&client).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

#[derive(Deserialize, Debug)]
struct GroupAward {
    ids: Vec<i64>,
    amount: f64,
}

#[post("/group_awards")]
async fn group_awards(
    mut data: web::Json<GroupAward>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    if is_no_permission_to_users(claim.role) {
        return Err(Error::NoPermission)
    }
    data.ids.sort();
    data.ids.dedup();

    user_info_model::award_money_by_id(&client, &data.ids, data.amount).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[derive(Deserialize, Debug)]
struct PermissionRequest {
    give: Vec<i32>,
    take: Vec<i32>,
    id: i64,
}

#[post("/change_permission")]
async fn change_permission(
    data: web::Json<PermissionRequest>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    if is_not_su(claim.role) {
        return Err(Error::NoPermission)
    }

    for permission in &data.give {
        user_model::add_role_by_id(&client, data.id, permission % 64).await?;
    }
    for permission in &data.take {
        user_model::delete_role_by_id(&client, data.id, permission % 64).await?;
    }
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

pub(crate) fn admin_service() -> Scope {
    web::scope("/admin")
        .service(web::scope("/torrent")
            .service(accept_torrents)
            .service(stick_torrents)
            .service(show_invisible_torrents))
        .service(web::scope("/user")
            .service(ban_user)
            .service(unban_user)
            .service(list_banned_user)
            .service(group_awards)
            .service(change_permission))
}