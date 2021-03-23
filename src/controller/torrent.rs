use super::*;
use crate::data::{user as user_model,
                  torrent as torrent_model,
                  torrent_info as torrent_info_model,
                  tag as tag_model};

#[derive(Deserialize, Debug)]
struct TorrentPost {
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
    let claim = get_info_in_token(req)?;
    let username = claim.sub;
    if is_not_ordinary_user(claim.role) {
        return Err(Error::NoPermission)
    }

    let tags = data.tags.as_deref().unwrap_or(&[]);
    if tags.len() > 5 {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("tags max amount is 5")))
    }
    let ret = torrent_info_model::add_torrent_info(&client, &data.title,&username, data.description.as_deref(), tags).await?;
    let mut tokens = vec![data.title.clone(), username];
    tokens.append(&mut data.tags.clone().unwrap_or_default());
    TORRENT_SEARCH_ENGINE.write().unwrap().insert(ret.id, tokens);
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

#[derive(Deserialize, Debug)]
struct TorrentUpdatePost {
    id: i64,
    title: String,
    description: Option<String>,
    tags: Option<Vec<String>>,
}

#[post("/update_torrent")]
async fn update_torrent(
    data: web::Json<TorrentUpdatePost>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    let username = claim.sub;

    let old_torrent = torrent_info_model::find_torrent_by_id_mini(&client, data.id).await?;
    if username != old_torrent.poster && is_no_permission_to_torrents(claim.role) {
        return Err(Error::NoPermission)
    }

    let tags = data.tags.as_deref().unwrap_or(&[]);
    if tags.len() > 5 {
        return Ok(HttpResponse::Ok().json(GeneralResponse::from_err("tags max amount is 5")))
    }
    let ret = torrent_info_model::update_torrent_info(&client, data.id, &data.title, data.description.as_deref(), tags).await?;
    let mut tokens = vec![data.title.clone(), username];
    tokens.append(&mut data.tags.clone().unwrap_or_default());
    TORRENT_SEARCH_ENGINE.write().unwrap().insert(ret.id, tokens);
    // tag count will only be updated when it is open
    if ret.visible {
        let old_tags = old_torrent.tag.unwrap_or_default();
        let to_decrease: Vec<&String> = old_tags.iter().filter(|tag| !tags.contains(tag)).collect();
        let to_increase: Vec<&String> = tags.iter().filter(|tag| !old_tags.contains(tag)).collect();
        for tag in to_decrease {
            tag_model::decrease_amount_by_name(&client, tag).await?;
        }
        for tag in to_increase {
            tag_model::update_or_add_tag(&client, tag).await?;
        }
    }
    Ok(HttpResponse::Ok().json(ret.to_json()))
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
enum Sort {
    Title,
    Poster,
    LastEdit,
    Length,
    Downloading,
    Uploading,
    Finished,
}

#[derive(Deserialize, Debug, PartialEq)]
enum SortType {
    ASC,
    DESC,
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Debug)]
struct ListRequest {
    tags: Option<Vec<String>>,
    page: Option<usize>,
    freeonly: bool,
    sort: Option<Sort>,
    #[serde(rename = "type")]
    sort_type: Option<SortType>,
}

/// list torrent with tags and pages
#[get("/list_torrents")]
async fn list_torrents(
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let query = req.uri().query().unwrap_or_default();
    let data: ListRequest = serde_qs::from_str(query).map_err(error_string)?;
    let tags = data.tags.unwrap_or_default();
    let page = data.page.unwrap_or(0);
    let sort = data.sort.unwrap_or(Sort::LastEdit);
    let sort_type = data.sort_type.unwrap_or(SortType::DESC);
    let sort_string = format!("{}", sort).to_ascii_lowercase();

    // by default you can add infinite number of stick torrents
    // but we recommend the number is less than 20
    let mut all_torrents = torrent_info_model::find_stick_torrent(&client).await?;
    let len = all_torrents.len();

    let count = torrent_info_model::query_torrent_counts_by_tag(&client, &tags).await? + len as i64;
    let mut ret = if sort_type == SortType::DESC {
        torrent_info_model::find_visible_torrent_by_tag_desc(&client, &tags, (page * 20 - len) as i64, &sort_string).await?
    } else {
        torrent_info_model::find_visible_torrent_by_tag_asc(&client, &tags, (page * 20 - len) as i64, &sort_string).await?
    };
    if data.freeonly {
        ret.retain(|t| t.free == true);
    }

    all_torrents.append(&mut ret);
    let resp = DataWithCount::new(serde_json::to_value(all_torrents).unwrap(), count / 20 + 1);
    Ok(HttpResponse::Ok().json(resp.to_json()))
}

#[derive(Deserialize, Debug)]
struct SearchRequest {
    keywords: Vec<String>,
    page: Option<usize>,
    freeonly: bool,
    sort: Option<Sort>,
    #[serde(rename = "type")]
    sort_type: Option<SortType>,
}

#[get("/search_torrents")]
async fn search_torrents(
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let query = req.uri().query().unwrap_or_default();
    let data: SearchRequest = serde_qs::from_str(query).map_err(error_string)?;
    let page = data.page.unwrap_or(0);
    let sort = data.sort.unwrap_or(Sort::LastEdit);
    let sort_type = data.sort_type.unwrap_or(SortType::DESC);
    let sort_string = format!("{}", sort).to_ascii_lowercase();

    let ids = TORRENT_SEARCH_ENGINE.read().unwrap()
        .search(data.keywords);
    let mut ret = if sort_type == SortType::DESC {
        torrent_info_model::find_visible_torrent_by_ids_desc(&client, &ids, (page * 20) as i64, &sort_string).await?
    } else {
        torrent_info_model::find_visible_torrent_by_ids_asc(&client, &ids, (page * 20) as i64, &sort_string).await?
    };
    if data.freeonly {
        ret.retain(|t| t.free == true);
    }

    Ok(HttpResponse::Ok().json(ret.to_json()))
}

#[get("list_posted_torrent")]
async fn list_posted_torrent(
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let username = get_name_in_token(req)?;
    let ret = torrent_info_model::find_torrent_by_poster(&client, &username).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

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
    if poster != username && is_no_permission_to_torrents(claim.role) {
        return Err(Error::NoPermission)
    }

    torrent_model::update_or_add_torrent(&client, &parsed.unwrap(), id).await?;

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[derive(Deserialize, Debug)]
struct IdWrapper {
    id: i64,
}

#[get("/show_torrent")]
async fn show_torrent(
    web::Query(data): web::Query<IdWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let ret = torrent_info_model::find_torrent_by_id(&client, data.id).await?;
    if !ret.visible {
        let claim = get_info_in_token(req)?;
        let username = claim.sub;
        if !ret.poster.eq(&username) && is_no_permission_to_torrents(claim.role) {
            return Err(Error::NoPermission)
        }
    }

    Ok(HttpResponse::Ok().json(ret.to_json()))
}

#[get("/get_torrent")]
async fn get_torrent(
    web::Query(data): web::Query<IdWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    let username = claim.sub;
    if is_not_ordinary_user(claim.role) {
        return Err(Error::NoPermission)
    }

    let user = user_model::find_user_by_username(&client, &username).await?;
    let torrent_info = torrent_info_model::find_torrent_by_id_mini(&client, data.id).await?;
    if !torrent_info.visible &&
        !username.eq(&torrent_info.poster) &&
        is_no_permission_to_torrents(claim.role) {
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
        .service(search_torrents)
        .service(show_torrent)
        .service(list_posted_torrent)
        .service(upload_torrent)
        .service(get_torrent)
}