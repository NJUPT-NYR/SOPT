use super::*;
use crate::data::{
    torrent_info as torrent_info_model, torrent_status as torrent_status_model, user as user_model,
    user_info as user_info_model,
};

#[repr(C)]
#[derive(Deserialize, Debug, Copy, Clone)]
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
    action: Option<Action>,
}

#[get("/get_announce")]
async fn get_announce(req: HttpRequest, client: web::Data<sqlx::PgPool>) -> HttpResult {
    use chrono::{Duration, Utc};

    let mut data = deserialize_from_req!(req, AnnouncePacket);
    let torrent = torrent_info_model::find_torrent_by_id_mini(&client, data.tid).await?;
    if torrent.free {
        data.download = 0;
    }
    let current_status =
        torrent_status_model::find_status_by_tid_uid(&client, data.tid, data.uid).await?;
    if !current_status.is_empty() {
        let status = current_status.first().unwrap();
        if status.status < 2 {
            user_info_model::update_money_by_id(
                &client,
                data.uid,
                0.4 * torrent.length as f64 / (1024_i64 ^ 3_i64) as f64,
            )
            .await?;
        }
    }
    let ret =
        user_info_model::update_io_by_id(&client, data.uid, data.upload, data.download).await?;
    let ratio = KVDB
        .clone()
        .get_float("config", "BAN USER RATIO".as_ref())?
        .unwrap();
    let days = KVDB
        .clone()
        .get_number("config", "NEWBIE TERM".as_ref())?
        .unwrap();
    if (ret.upload as f64 / ret.download as f64) < ratio
        && (Utc::now() - Duration::days(days)).timestamp() > ret.registertime.timestamp()
    {
        user_model::delete_role_by_id(&client, data.uid, 0).await?;
    }

    let status = data.action.clone().unwrap_or(Action::Start) as i32;
    torrent_status_model::update_or_add_status(
        &client,
        data.tid,
        data.uid,
        status,
        data.upload,
        data.download,
    )
    .await?;
    if data.action.is_some() {
        match data.action.unwrap() {
            Action::Start => {
                torrent_info_model::update_torrent_status(&client, data.tid, 1, 1, 0).await?
            }
            Action::Complete => {
                torrent_info_model::update_torrent_status(&client, data.tid, -1, 0, 1).await?;
                torrent_status_model::update_finished_by_tid_uid(&client, data.tid, data.uid)
                    .await?;
            }
            Action::Stop => {
                torrent_info_model::update_torrent_status(&client, data.tid, -1, -1, 0).await?
            }
        }
    }

    Ok(HttpResponse::Ok().json(GeneralResponse::default()))
}

pub(crate) fn tracker_service() -> Scope {
    web::scope("/tracker").service(get_announce)
}
