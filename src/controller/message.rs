use super::*;
use crate::data::message as message_model;

#[post("/send_message")]
async fn send_message(
    data: web::Json<MessageRequest>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(&req)?;
    let sender = claim.sub;
    if is_not_ordinary_user(claim.role) || cannot_send_msg(claim.role) {
        return Err(Error::NoPermission);
    }

    message_model::add_message(
        &client,
        &sender,
        &data.receiver,
        &data.title,
        data.body.as_deref(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[post("/read_message")]
async fn read_message(
    data: web::Json<IdsWrapper>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let receiver = get_name_in_token(&req)?;
    message_model::read_message(&client, &data.ids, &receiver).await?;
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[post("/delete_message")]
async fn delete_message(
    data: web::Json<MessageDeleteRequest>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let username = get_name_in_token(&req)?;
    if data.sender {
        message_model::delete_message_by_sender(&client, &data.ids, &username).await?;
    } else {
        message_model::delete_message_by_receiver(&client, &data.ids, &username).await?;
    }
    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

#[get("/list_sent")]
async fn list_sent(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let sender = get_name_in_token(&req)?;
    let ret = message_model::list_sent_message(&client, &sender).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

#[get("/list_received")]
async fn list_received(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    let receiver = get_name_in_token(&req)?;
    let ret = message_model::list_received_message(&client, &receiver).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

pub(crate) fn message_service() -> Scope {
    web::scope("/message")
        .service(send_message)
        .service(read_message)
        .service(delete_message)
        .service(list_sent)
        .service(list_received)
}
