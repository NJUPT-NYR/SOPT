use super::*;

pub(crate) type TorrentIdRet = Result<TorrentId, Error>;
pub(crate) type MiniTorrentRet = Result<MiniTorrent, Error>;
pub(crate) type MiniTorrentVecRet = Result<Vec<MiniTorrent>, Error>;
pub(crate) type SlimTorrentVecRet = Result<Vec<SlimTorrent>, Error>;
pub(crate) type FullTorrentRet = Result<FullTorrent, Error>;

pub(crate) type TagVecRet = Result<Vec<Tag>, Error>;

pub(crate) type TorrentStatusVecRet = Result<Vec<TorrentStatus>, Error>;
pub(crate) type PersonalTorrentVecRet = Result<Vec<PersonalTorrent>, Error>;

#[derive(Serialize, Debug, ToResponse)]
pub struct TorrentId {
    pub id: i64,
    pub visible: bool,
}

#[derive(Debug)]
pub struct MiniTorrent {
    pub poster: String,
    pub visible: bool,
    pub free: bool,
    pub tag: Option<Vec<String>>,
    pub length: i64,
}

#[derive(Serialize, Debug, ToResponse)]
pub struct SlimTorrent {
    pub id: i64,
    pub title: String,
    pub poster: String,
    pub tag: Option<Vec<String>>,
    #[serde(rename = "lastEdit")]
    pub lastedit: DateTime<Utc>,
    pub length: i64,
    pub free: bool,
    pub downloading: i32,
    pub uploading: i32,
    pub finished: i64,
}

#[derive(Serialize, Debug, ToResponse)]
pub struct FullTorrent {
    pub id: i64,
    pub title: String,
    pub poster: String,
    pub description: Option<String>,
    pub visible: bool,
    pub tag: Option<Vec<String>>,
    #[serde(rename = "createTime")]
    pub createtime: DateTime<Utc>,
    #[serde(rename = "lastEdit")]
    pub lastedit: DateTime<Utc>,
    pub free: bool,
    pub downloading: i32,
    pub uploading: i32,
    pub finished: i64,
    pub length: Option<i64>,
    pub files: Option<Vec<String>>,
    pub infohash: Option<String>,
}

#[derive(Serialize, Debug, ToResponse)]
pub struct Tag {
    pub name: String,
    pub amount: i32,
}

#[derive(Debug)]
pub struct TorrentStatus {
    pub tid: i64,
    pub uid: i64,
    pub status: i32,
    pub upload: i64,
    pub download: i64,
    pub finished: bool,
}

#[derive(Serialize, Debug, ToResponse)]
pub struct PersonalTorrent {
    pub id: i64,
    pub title: String,
    pub length: i64,
    pub upload: i64,
    pub download: i64,
    pub free: bool,
}

#[derive(Serialize, Debug, ToResponse)]
pub struct TorrentStatusByUser {
    pub uploading: Vec<PersonalTorrent>,
    pub downloading: Vec<PersonalTorrent>,
    pub finished: Vec<PersonalTorrent>,
    pub unfinished: Vec<PersonalTorrent>,
}
