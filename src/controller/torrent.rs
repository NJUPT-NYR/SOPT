use super::*;
use crate::data::{user as user_model,
                  torrent as torrent_model,
                  torrent_info as torrent_info_model,
                  tag as tag_model};

#[derive(Deserialize, Debug)]
struct TorrentPost {
    id: Option<i64>,
    title: String,
    description: Option<String>,
    tags: Option<Vec<String>>,
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
    let claim = get_info_in_token(req)?;
    let username = claim.sub;

    if claim.role & 1 == 0 {
        return Err(Error::NoPermission)
    }

    let ret = torrent_info_model::add_torrent_info(&client, &post.title,&username, post.description).await?;
    if post.tags.is_some() {
        let tags = post.tags.unwrap();
        if tags.len() > 5 {
            return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("tags max amount is 5")))
        }
        let new_ret = torrent_info_model::add_tag_for_torrent(&client, ret.id, &tags).await?;
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
    let claim = get_info_in_token(req)?;
    let username = claim.sub;
    if post.id.is_none() {
        return Ok(HttpResponse::BadRequest().json(GeneralResponse::from_err("missing torrent id")))
    }

    let old_torrent = torrent_info_model::find_torrent_by_id(&client,post.id.unwrap()).await?;
    let poster = old_torrent.poster;
    if !username.eq(&poster) && claim.role & (1 << 62) == 0 {
        return Err(Error::NoPermission)
    }

    let ret = torrent_info_model::update_torrent_info(&client,post.id.unwrap(), &post.title, post.description).await?;
    if post.tags.is_some() {
        let tags = post.tags.unwrap();
        if tags.len() > 5 {
            return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("tags max amount is 5")))
        }
        let new_ret = torrent_info_model::add_tag_for_torrent(&client, ret.id, &tags).await?;

        // tag count will only be updated when it is open
        if ret.visible {
            let old_tags = old_torrent.tag.unwrap_or(vec![]);
            let to_decrease: Vec<&String> = old_tags.iter().filter(|tag| !tags.contains(tag)).collect();
            let to_increase: Vec<&String> = tags.iter().filter(|tag| !old_tags.contains(tag)).collect();
            for tag in to_decrease {
                tag_model::decrease_amount_by_name(&client, tag).await?;
            }
            for tag in to_increase {
                tag_model::update_or_add_tag(&client, tag).await?;
            }
        }

        Ok(HttpResponse::Ok().json(new_ret.to_json()))
    } else {
        Ok(HttpResponse::Ok().json(ret.to_json()))
    }
}

#[derive(Deserialize, Debug)]
struct TagRequest {
    num: Option<usize>,
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

#[derive(Deserialize, Debug)]
struct ListRequest {
    tags: Option<Vec<String>>,
    page: Option<usize>,
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
    let claim = get_info_in_token(req)?;
    let username = claim.sub;
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

    let claim = get_info_in_token(req)?;
    let username = claim.sub;
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
    let poster = torrent_info_model::find_torrent_by_id_mini(&client, id).await?.poster;
    if poster != username && claim.role & (1 << 62) == 0 {
        return Err(Error::NoPermission)
    }

    torrent_model::update_or_add_torrent(&client, &parsed.unwrap(), id).await?;

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[derive(Deserialize, Debug)]
struct IdWrapper {
    id: i64,
}

/// show definite torrent with an id
#[get("/show_torrent")]
async fn show_torrent(
    web::Query(data): web::Query<IdWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let info = torrent_info_model::find_torrent_by_id(&client, data.id).await?;
    let torrent = torrent_model::find_slim_torrent_by_id(&client, data.id).await?;

    if !info.visible {
        let claim = get_info_in_token(req)?;
        let username = claim.sub;
        if !info.poster.eq(&username) {
            return Err(Error::NoPermission)
        }
    }

    let ret = torrent_info_model::JoinedTorrent {
        info,
        torrent,
    };
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

/// download torrent with passkey in it
#[get("/get_torrent")]
async fn get_torrent(
    web::Query(data): web::Query<IdWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    let username = claim.sub;

    if claim.role & 1 == 0 {
        return Err(Error::NoPermission)
    }

    let user = user_model::find_user_by_username(&client, &username).await?.pop().unwrap();
    let torrent_info = torrent_info_model::find_torrent_by_id_mini(&client, data.id).await?;
    if !torrent_info.visible &&
        !username.eq(&torrent_info.poster) &&
        claim.role & (1 << 62) == 0 {
        return Err(Error::NoPermission)
    }

    let torrent = torrent_model::find_torrent_by_id(&client, data.id).await?;
    let generated_torrent = generate_torrent_file(
        torrent.info,
        &user.passkey,
        torrent.id,
        user.id,
        &torrent.comment.unwrap_or_default(),
    );

    Ok(HttpResponse::Ok()
        .header(
            http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}.torrent\"", torrent.name))
        .content_type("application/octet-stream")
        .body(body::Body::from_slice(&generated_torrent)))
}

pub(crate) fn torrent_service() -> Scope {
    web::scope("/torrent")
        .service(add_torrent)
        .service(update_torrent)
        .service(hot_tags)
        .service(list_torrents)
        .service(show_torrent)
        .service(list_posted_torrent)
        .service(upload_torrent)
        .service(get_torrent)
}