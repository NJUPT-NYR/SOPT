use super::*;
use crate::data::{
    rank as rank_model, tag as tag_model, torrent_info as torrent_info_model, user as user_model,
    user_info as user_info_model,
};

#[get("/show_invisible_torrents")]
async fn show_invisible_torrents(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_torrents(claim.role) {
        return Err(Error::NoPermission);
    }
    let ret = torrent_info_model::find_invisible_torrent(&client).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

#[post("/accept_torrents")]
async fn accept_torrents(
    data: web::Json<IdsWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_torrents(claim.role) {
        return Err(Error::NoPermission);
    }
    let ret = torrent_info_model::make_torrent_visible(&client, &data.ids).await?;

    for torrent in ret {
        for tag in torrent.tag.unwrap_or_default() {
            tag_model::update_or_add_tag(&client, &tag).await?;
        }
    }
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[post("/stick_torrents")]
async fn stick_torrents(
    data: web::Json<IdsWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_torrents(claim.role) {
        return Err(Error::NoPermission);
    }
    torrent_info_model::make_torrent_stick(&client, &data.ids).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[post("/unstick_torrents")]
async fn unstick_torrents(
    data: web::Json<IdsWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_torrents(claim.role) {
        return Err(Error::NoPermission);
    }
    torrent_info_model::make_torrent_unstick(&client, &data.ids).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[post("/free_torrents")]
async fn free_torrents(
    data: web::Json<IdsWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_torrents(claim.role) {
        return Err(Error::NoPermission);
    }
    torrent_info_model::make_torrent_free(&client, &data.ids).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[post("/unfree_torrents")]
async fn unfree_torrents(
    data: web::Json<IdsWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_torrents(claim.role) {
        return Err(Error::NoPermission);
    }
    torrent_info_model::make_torrent_unfree(&client, &data.ids).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

/// I hope this never get used
#[get("/ban_user")]
async fn ban_user(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_users(claim.role) {
        return Err(Error::NoPermission);
    }
    let data = deserialize_from_req!(req, IdWrapper);
    let old_key = user_model::find_user_by_id(&client, data.id).await?.passkey;
    user_model::delete_role_by_id(&client, data.id, 0).await?;
    update_passkey_filter(None, Some(old_key)).await?;

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[get("/unban_user")]
async fn unban_user(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_users(claim.role) {
        return Err(Error::NoPermission);
    }
    let data = deserialize_from_req!(req, IdWrapper);
    let old_key = user_model::find_user_by_id(&client, data.id).await?.passkey;
    user_model::add_role_by_id(&client, data.id, 0).await?;
    update_passkey_filter(None, Some(old_key)).await?;

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[get("/list_banned_user")]
async fn list_banned_user(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_users(claim.role) {
        return Err(Error::NoPermission);
    }
    let ret = user_model::list_banned_user(&client).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

#[post("/group_awards")]
async fn group_awards(
    mut data: web::Json<GroupAwardRequest>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_users(claim.role) {
        return Err(Error::NoPermission);
    }
    data.ids.sort_unstable();
    data.ids.dedup();

    user_info_model::award_money_by_id(&client, &data.ids, data.amount).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[post("/change_permission")]
async fn change_permission(
    data: web::Json<PermissionRequest>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_not_su(claim.role) {
        return Err(Error::NoPermission);
    }

    for permission in &data.give {
        user_model::add_role_by_id(&client, data.id, permission % 64).await?;
    }
    for permission in &data.take {
        user_model::delete_role_by_id(&client, data.id, permission % 64).await?;
    }
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[get("/award_rank")]
async fn award_rank(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_users(claim.role) {
        return Err(Error::NoPermission);
    }

    let data = deserialize_from_req!(req, RankAwardRequest);
    let rank = rank_model::find_rank_by_id(&client, data.rid).await?;
    let roles = rank.role;
    for role in roles {
        user_model::add_role_by_id(&client, data.uid, (role % 32) as i32).await?;
    }
    user_info_model::update_rank_by_id(&client, data.uid, rank.id).await?;

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[get("/get_email_whitelist")]
async fn get_email_whitelist(req: HttpRequest) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_site(claim.role) {
        return Err(Error::NoPermission);
    }
    let ret = ALLOWED_DOMAIN.read().await;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

#[post("/update_email_whitelist")]
async fn update_email_whitelist(data: web::Json<EmailListRequest>, req: HttpRequest) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_site(claim.role) {
        return Err(Error::NoPermission);
    }

    let mut w = ALLOWED_DOMAIN.write().await;
    data.add.iter().for_each(|s| {
        w.insert(String::from(s));
    });
    data.delete.iter().for_each(|s| {
        w.take(s);
    });
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[get("/get_rank")]
async fn get_rank(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_site(claim.role) {
        return Err(Error::NoPermission);
    }
    let ret = rank_model::find_all_ranks(&client).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

#[post("/update_rank")]
async fn update_rank(
    data: web::Json<Rank>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_site(claim.role) {
        return Err(Error::NoPermission);
    }
    rank_model::update_or_add_rank(&client, data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[get("/list_site_settings")]
async fn list_site_settings(req: HttpRequest) -> HttpResult {
    use std::collections::HashMap;

    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_site(claim.role) {
        return Err(Error::NoPermission);
    }
    let mut settings: HashMap<String, String> = HashMap::new();
    for (key, _) in STRING_SITE_SETTING.iter() {
        let setting = KVDB.clone().get_string("config", key.as_ref())?.unwrap();
        settings.insert(key.to_string(), setting);
    }
    let val = KVDB
        .clone()
        .get_float("config", "INVITE CONSUME".as_ref())?
        .unwrap();
    settings.insert("INVITE CONSUME".to_string(), val.to_string());
    let val = KVDB
        .clone()
        .get_float("config", "BAN UPLOAD RATIO".as_ref())?
        .unwrap();
    settings.insert("BAN UPLOAD RATIO".to_string(), val.to_string());
    let val = KVDB
        .clone()
        .get_number("config", "NEWBIE TERM".as_ref())?
        .unwrap();
    settings.insert("NEWBIE TERM".to_string(), val.to_string());
    let val = KVDB
        .clone()
        .get_number("config", "LOGIN EXPIRE DAY".as_ref())?
        .unwrap();
    settings.insert("LOGIN EXPIRE DAY".to_string(), val.to_string());

    Ok(HttpResponse::Ok().json(settings.to_json()))
}

#[post("/update_site_settings")]
async fn update_site_settings(data: web::Json<SiteSettingRequest>, req: HttpRequest) -> HttpResult {
    use std::str::FromStr;

    let claim = get_info_in_token(&req)?;
    if is_no_permission_to_site(claim.role) {
        return Err(Error::NoPermission);
    }
    for (key, val) in data.settings.iter() {
        if key.eq("INVITE CONSUME") {
            KVDB.clone().put(
                "config",
                "INVITE CONSUME".as_ref(),
                &f64::from_str(&val).map_err(error_string)?.to_ne_bytes(),
            )?;
        }
        if key.eq("BAN UPLOAD RATIO") {
            KVDB.clone().put(
                "config",
                "BAN UPLOAD RATIO".as_ref(),
                &f64::from_str(&val).map_err(error_string)?.to_ne_bytes(),
            )?;
        }
        if key.eq("NEWBIE TERM") {
            KVDB.clone().put(
                "config",
                "NEWBIE TERM".as_ref(),
                &i64::from_str(&val).map_err(error_string)?.to_ne_bytes(),
            )?;
        }
        if key.eq("LOGIN EXPIRE DAY") {
            KVDB.clone().put(
                "config",
                "LOGIN EXPIRE DAY".as_ref(),
                &i64::from_str(&val).map_err(error_string)?.to_ne_bytes(),
            )?;
        }
        if STRING_SITE_SETTING
            .keys()
            .find(|x| x.to_string().eq(key))
            .is_some()
        {
            KVDB.clone().put("config", key.as_ref(), val.as_ref())?;
        }
    }

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

pub(crate) fn admin_service() -> Scope {
    web::scope("/admin")
        .service(
            web::scope("/torrent")
                .service(accept_torrents)
                .service(stick_torrents)
                .service(unstick_torrents)
                .service(free_torrents)
                .service(unfree_torrents)
                .service(show_invisible_torrents),
        )
        .service(
            web::scope("/user")
                .service(ban_user)
                .service(unban_user)
                .service(list_banned_user)
                .service(group_awards)
                .service(change_permission)
                .service(award_rank),
        )
        .service(
            web::scope("/site")
                .service(get_email_whitelist)
                .service(update_email_whitelist)
                .service(get_rank)
                .service(update_rank)
                .service(list_site_settings)
                .service(update_site_settings),
        )
}
