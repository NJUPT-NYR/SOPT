use super::*;
use crate::data::{ToResponse, GeneralResponse, DataWithCount,
                  torrent as torrent_model,
                  torrent_info as torrent_info_model,
                  tag as tag_model};

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

#[derive(Deserialize, Debug)]
struct TagRequest {
    pub num: Option<usize>,
}

/// add a post for definite torrent
/// by default this post is invisible
#[post("/add_torrent")]
async fn add_torrent(
    data: web::Json<TorrentPost>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let post: TorrentPost = data.into_inner();
    let username = get_name_in_token(req)?;

    let ret = torrent_info_model::add_torrent_info(&client,
                                                   torrent_info_model::TorrentInfo::new(
                                                       post.title,
                                                       username,
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
        // async closure is unstable now
        for tag in tags {
            tag_model::update_or_add_tag(&client, &tag).await?;
        }

        Ok(HttpResponse::Ok().json(new_ret.to_json()))
    } else {
        Ok(HttpResponse::Ok().json(ret.to_json()))
    }
}

/// update a post, like setting tags and add descriptions
#[post("/update_torrent")]
async fn update_torrent(
    data: web::Json<TorrentPost>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let post: TorrentPost = data.into_inner();
    let username = get_name_in_token(req)?;
    if post.id.is_none() {
        return Ok(HttpResponse::BadRequest().json(GeneralResponse::from_err("missing torrent id")))
    }

    let old_torrent = torrent_info_model::find_torrent_by_id(&client, post.id.unwrap()).await?;
    let poster = old_torrent.poster;
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
        let old_tags = old_torrent.tag.unwrap_or(vec![]);
        let to_decrease: Vec<&String> = old_tags.iter().filter(|tag| !tags.contains(tag)).collect();
        let to_increase: Vec<&String> = tags.iter().filter(|tag| !old_tags.contains(tag)).collect();

        let new_ret =
            torrent_info_model::add_tag_for_torrent(&client, ret.id, &tags)
                .await?;
        for tag in to_decrease {
            tag_model::decrease_amount_by_name(&client, tag).await?;
        }
        for tag in to_increase {
            tag_model::update_or_add_tag(&client, tag).await?;
        }

        Ok(HttpResponse::Ok().json(new_ret.to_json()))
    } else {
        Ok(HttpResponse::Ok().json(ret.to_json()))
    }
}

/// Get hottest tags by amount, default number is 10
#[get("/hot_tags")]
async fn hot_tags(
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let query = req.uri().query().unwrap_or_default();
    let num_want = serde_qs::from_str::<TagRequest>(query)
        .map_err(error_string)?.num.unwrap_or(10);

    let ret = tag_model::find_hot_tag_by_amount(&client, num_want as i64).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

/// list torrent with tags and pages
#[get("/list_torrents")]
async fn list_torrents(
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let query = req.uri().query().unwrap_or_default();
    let data: QueryList = serde_qs::from_str(query).map_err(error_string)?;
    let tags = data.tags;
    let page = data.page.unwrap_or(0);

    // by default you can add infinite number of stick torrents
    // but we recommend the number is less than 20
    let mut all_torrents = torrent_info_model::find_stick_torrent(&client).await?;
    let len = all_torrents.len();

    if tags.is_none() {
        let count = torrent_info_model::query_torrent_counts(&client).await? + len as i64;
        let mut ret = torrent_info_model::find_visible_torrent(&client, (page * 20 - len) as i64).await?;
        all_torrents.append(&mut ret);
        let resp = DataWithCount::new(serde_json::to_value(all_torrents).unwrap(), count / 20 + 1);
        Ok(HttpResponse::Ok().json(resp.to_json()))
    } else {
        let tags = tags.unwrap();
        let count = torrent_info_model::query_torrent_counts_by_tag(&client, &tags).await? + len as i64;
        let mut ret = torrent_info_model::find_visible_torrent_by_tag(&client, &tags, (page * 20 - len) as i64).await?;
        all_torrents.append(&mut ret);
        let resp = DataWithCount::new(serde_json::to_value(all_torrents).unwrap(), count / 20 + 1);
        Ok(HttpResponse::Ok().json(resp.to_json()))
    }
}

/// list all torrents current user posted
#[get("list_posted_torrent")]
async fn list_posted_torrent(
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let username = get_name_in_token(req)?;
    let ret = torrent_info_model::find_torrent_by_poster(&client, username).await?;

    Ok(HttpResponse::Ok().json(ret.to_json()))
}

/// upload torrent file and parse to database column
#[post("/upload_torrent")]
async fn upload_torrent(
    mut payload: actix_multipart::Multipart,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    use futures::{StreamExt, TryStreamExt};
    use std::str::FromStr;
    use std::collections::HashMap;

    let username = get_name_in_token(req)?;
    let mut parsed = None;
    let mut hash_map = HashMap::new();

    while let Ok(Some(mut file)) = payload.try_next().await {
        let content_type = file.content_disposition().ok_or(Error::OtherError("incomplete file".to_string()))?;
        let name = content_type.get_name().ok_or("incomplete file".to_string())?;
        let mut buf: Vec<u8> = vec![];
        while let Some(chunk) = file.next().await {
            let data = chunk.unwrap();
            buf.append(&mut data.to_vec());
        }
        if name.is_empty() {
            parsed = Some(parse_torrent_file(&buf)?);
        } else {
            hash_map.insert(name.to_string(), String::from_utf8(buf).unwrap());
        }
    }

    if parsed.is_none() {
        return Ok(HttpResponse::BadRequest().json(GeneralResponse::from_err("missing torrent file")))
    }
    let id_string = hash_map.get("id").ok_or(Error::OtherError("missing id field".to_string()))?;
    let id = i64::from_str(id_string).map_err(error_string)?;
    torrent_model::update_or_add_torrent(&client, &parsed.unwrap(), id).await?;

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

/// show definite torrent with an id
#[get("/show_torrent")]
async fn show_torrent(
    web::Query(data): web::Query<DetailRequest>,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let ret = torrent_info_model::find_torrent_by_id(&client, data.id).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

pub fn torrent_service() -> Scope {
    web::scope("/torrent")
        .service(add_torrent)
        .service(update_torrent)
        .service(hot_tags)
        .service(list_torrents)
        .service(show_torrent)
        .service(list_posted_torrent)
        .service(upload_torrent)
}