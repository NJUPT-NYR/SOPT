use super::*;
use crate::data::{invitation as invitation_model};

#[derive(Deserialize, Debug)]
struct Message {
    to: String,
    address: String,
    body: String,
}

/// consume money and send a mail
/// with another thread.
#[post("/send_invitation")]
async fn send_invitation(
    data: web::Json<Message>,
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let message: Message = data.into_inner();
    let claim = get_info_in_token(req)?;
    let username = claim.sub;

    if claim.role & (1 << 1) == 0 {
        return Err(Error::NoPermission)
    }

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

    // TODO: some consumption of money(site_general settings)
    let ret = invitation_model::add_invitation_code(&client, &username, &code, &message.address).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

/// list all invitations sent by current user
#[get("/list_invitations")]
async fn list_invitations(
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let claim = get_info_in_token(req)?;
    let username = claim.sub;

    let ret = invitation_model::find_invitation_by_user(&client, &username).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

pub(crate) fn invitation_service() -> Scope {
    web::scope("/invitation")
        .service(send_invitation)
        .service(list_invitations)
}
