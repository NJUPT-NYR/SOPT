use actix_web::{HttpResponse, *};
use actix_identity::Identity;
use serde::Deserialize;
use super::*;
use crate::data::{ToResponse, torrent_info as torrent_info_model, GeneralResponse, DataWithCount};
use crate::error::Error;

#[derive(Deserialize, Debug)]
struct TorrentPost {
    pub id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct QueryList {
    pub tags: Option<Vec<String>>,
    pub page: Option<usize>,
}

#[derive(Deserialize, Debug)]
struct DetailRequest {
    pub id: i64,
}

/// add a post for definite torrent
/// by default this post is invisible
#[post("/add_torrent")]
async fn add_torrent(
    data: web::Json<TorrentPost>,
    id: Identity,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let post: TorrentPost = data.into_inner();
    let poster = id.identity().ok_or(Error::CookieError)?;

    // TODO: check qualification
    let ret = torrent_info_model::add_torrent_info(&client,
                                                   torrent_info_model::TorrentInfo::new(
                                                       post.title,
                                                       poster,
                                                       post.description
                                                   )).await?;
    // TODO: eliminate duplication codes
    if post.tags.is_some() {
        let tags = post.tags.unwrap();
        if tags.len() > 5 {
            return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("tags max amount is 5")))
        }
        let new_ret =
            torrent_info_model::add_tag_for_torrent(&client, ret.id, &tags)
                .await?;
        Ok(HttpResponse::Ok().json(new_ret.to_json()))
    } else {
        Ok(HttpResponse::Ok().json(ret.to_json()))
    }
}

/// update a post, like setting tags and add descriptions
#[post("/update_torrent")]
async fn update_torrent(
    data: web::Json<TorrentPost>,
    id: Identity,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let post: TorrentPost = data.into_inner();
    let username = id.identity().ok_or(Error::CookieError)?;
    if post.id.is_none() {
        return Ok(HttpResponse::BadRequest().json(GeneralResponse::from_err("missing torrent id")))
    }

    let poster = torrent_info_model::find_torrent_by_id(&client, post.id.unwrap()).await?.poster;
    if !username.eq(&poster) {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("not the owner of post")))
    }

    let ret = torrent_info_model::update_torrent_info(&client,
                                                      post.id.unwrap(),
                                                      torrent_info_model::TorrentInfo::new(
                                                          post.title,
                                                          poster,
                                                          post.description
                                                      )).await?;
    if post.tags.is_some() {
        let tags = post.tags.unwrap();
        if tags.len() > 5 {
            return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("tags max amount is 5")))
        }
        let new_ret =
            torrent_info_model::add_tag_for_torrent(&client, ret.id, &tags)
                .await?;
        Ok(HttpResponse::Ok().json(new_ret.to_json()))
    } else {
        Ok(HttpResponse::Ok().json(ret.to_json()))
    }
}

/// list torrent with tags and pages
#[get("/list_torrents")]
async fn list_torrents(
    web::Query(data): web::Query<QueryList>,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    // TODO: is it better to show available tags to users?
    // TODO: Custom Page Offset(Draft)
    let tags: Option<Vec<String>> = data.tags;
    // is it safe?
    let page: usize = data.page.unwrap_or(0);

    if tags.is_none() {
        let count = torrent_info_model::query_torrent_counts(&client).await?;
        let ret = torrent_info_model::find_visible_torrent(&client, (page * 20) as i64).await?;
        let resp = DataWithCount::new(serde_json::to_value(ret).unwrap(), count / 20 + 1);
        Ok(HttpResponse::Ok().json(resp.to_json()))
    } else {
        let tags = tags.unwrap();
        if tags.len() == 0 {
            return Ok(HttpResponse::BadRequest().json(GeneralResponse::from_err("tags are empty")))
        }

        let count = torrent_info_model::query_torrent_counts_by_tag(&client, &tags).await?;
        let ret = torrent_info_model::find_visible_torrent_by_tag(&client, &tags, (page * 20) as i64).await?;
        let resp = DataWithCount::new(serde_json::to_value(ret).unwrap(), count / 20 + 1);
        Ok(HttpResponse::Ok().json(resp.to_json()))
    }
}

/// list all torrents current user posted
#[get("list_posted_torrent")]
async fn list_posted_torrent(
    id: Identity,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let poster = id.identity().ok_or(Error::CookieError)?;
    let ret = torrent_info_model::find_torrent_by_poster(&client, poster).await?;

    Ok(HttpResponse::Ok().json(ret.to_json()))
}

/// upload torrent file and parse to database column
#[post("/upload_torrent")]
async fn upload_torrent(

) -> HttpResult {
    todo!()
}

/// show definite torrent with an id
#[get("/show_torrent")]
async fn show_torrent(
    web::Query(data): web::Query<DetailRequest>,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let id: i64 = data.id;
    let ret = torrent_info_model::find_torrent_by_id(&client, id).await?;

    Ok(HttpResponse::Ok().json(ret.to_json()))
}

pub fn torrent_service() -> Scope {
    web::scope("/torrent")
        .service(add_torrent)
        .service(update_torrent)
        .service(list_torrents)
        .service(show_torrent)
        .service(list_posted_torrent)
}