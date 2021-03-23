use super::*;
use crate::data::{invitation as invitation_model,
                  user_info as user_info_model};

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
    let claim = get_info_in_token(req)?;
    let username = claim.sub;
    if is_not_ordinary_user(claim.role) || cannot_invite(claim.role) {
        return Err(Error::NoPermission)
    }

    let code = generate_invitation_code();
    let num = get_from_rocksdb!("INVITE_CONSUME", f64);
    user_info_model::update_money_by_name(&client, &username, num).await?;
    let ret = invitation_model::add_invitation_code(&client, &username, &code, &data.address).await?;
    // we don't really care about the result of send mail
    std::thread::spawn(move || {
       send_mail(
            &username,
            &data.address,
            &data.to,
            format!("{}\n\nYour Invitation Code is: {}\n", &data.body, &code),
       ).expect("unable to send mail");
    });
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

#[get("/list_invitations")]
async fn list_invitations(
    req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    let username = get_name_in_token(req)?;
    let ret = invitation_model::find_invitation_by_user(&client, &username).await?;
    Ok(HttpResponse::Ok().json(ret.to_json()))
}

pub(crate) fn invitation_service() -> Scope {
    web::scope("/invitation")
        .service(send_invitation)
        .service(list_invitations)
}
