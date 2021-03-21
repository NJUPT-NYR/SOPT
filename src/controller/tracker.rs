use super::*;
use crate::data::{user as user_model,
                  user_info as user_info_model,
                  torrent_info as torrent_info_model};

#[repr(C)]
#[derive(Deserialize, Debug)]
enum Action {
    Start = 0,
    Complete,
    Stop,
}

#[derive(Deserialize, Debug)]
struct AnnouncePacket {
    uid: i64,
    tid: i64,
    download: i64,
    upload: i64,
    action: Action,
}

#[get("/get_announce")]
async fn get_announce(
    web::Query(mut data): web::Query<AnnouncePacket>,
    // req: HttpRequest,
    client: web::Data<sqlx::PgPool>,
) -> HttpResult {
    use chrono::{Utc, Duration};
    // TODO: identity check

    let torrent = torrent_info_model::find_torrent_by_id_mini(&client, data.tid).await?;
    if torrent.free {
        data.download = 0;
    }
    // TODO: add money and warn user about ratio
    let ret = user_info_model::update_io_by_id(&client, data.uid, data.upload, data.download).await?;
    if (ret.upload as f64 / ret.download as f64) < BAN_UPLOAD_RATIO &&
        (Utc::now() - Duration::days(14)).timestamp() > ret.register_time.timestamp() {
        user_model::delete_role_by_id(&client, data.uid, 0).await?;
    }

    match data.action {
        Action::Start => torrent_info_model::update_torrent_status(&client, data.tid, 1, 1, 0).await?,
        Action::Complete => torrent_info_model::update_torrent_status(&client, data.tid, -1, 0, 1).await?,
        Action::Stop => torrent_info_model::update_torrent_status(&client, data.tid, -1, -1, 0).await?,
    }

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

pub(crate) fn tracker_service() -> Scope {
    web::scope("/tracker")
        .service(get_announce)
}