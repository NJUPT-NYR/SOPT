use actix_web::{HttpResponse, web, *};
use actix_identity::Identity;
use serde::Deserialize;
use super::*;
use crate::util::*;
use crate::data::invitation as invitation_model;
use crate::error::Error;

#[derive(Deserialize, Debug)]
struct Message {
    pub to: String,
    pub address: String,
    pub body: String,
}

#[post("/send_invitation")]
async fn send_invitation(
    data: web::Json<Message>,
    id: Identity,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let message: Message = data.into_inner();
    let username = id.identity().ok_or(Error::CookieError)?;

    let code = generate_invitation_code();
    let body = format!("{}\n\nYour Invitation Code is: {}\n", &message.body, &code);
    // fuck u borrow checker
    let from = username.clone();
    let address = message.address.clone();
    let receiver = message.to.clone();
    // we don't really care about the result of send mail
    std::thread::spawn(move || {
       send_mail(
            receiver,
            address,
            from,
            body,
       ).expect("unable to send mail");
    });

    // TODO: some consumption of money
    let ret = invitation_model::add_invitation_code(
            &client,
            invitation_model::InvitationCode::new(
                username,
                code,
                message.address,
            )).await?;
    Ok(HttpResponse::Ok().json(&ret))
}

#[get("/list_invitations")]
async fn list_invitations(
    id: Identity,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let username = id.identity().ok_or(Error::CookieError)?;

    let ret = invitation_model::find_invitation_by_user(&client, &username)
            .await?;
    Ok(HttpResponse::Ok().json(&ret))
}

pub fn invitation_service() -> Scope {
    web::scope("/invitation")
        .service(send_invitation)
        .service(list_invitations)
}
