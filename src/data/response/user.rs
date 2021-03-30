use super::*;

pub type AccountRet = Result<Account, Error>;
pub type AccountVecRet = Result<Vec<Account>, Error>;
pub type ValidationVecRet = Result<Vec<Validation>, Error>;

pub type MiniUserRet = Result<MiniUser, Error>;
pub type UserRet = Result<User, Error>;

pub type InvitationRet = Result<Invitation, Error>;
pub type InvitationVecRet = Result<Vec<Invitation>, Error>;

pub type RankRet = Result<Rank, Error>;
pub type RankVecRet = Result<Vec<Rank>, Error>;

pub type MessageVecRet = Result<Vec<Message>, Error>;

#[derive(Serialize, Debug, ToResponse)]
pub struct Account {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub passkey: String,
    pub role: i64,
}

#[derive(Debug)]
pub struct Validation {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub role: i64,
}

#[derive(Debug)]
pub struct MiniUser {
    pub id: i64,
    pub registertime: DateTime<Utc>,
    pub upload: i64,
    pub download: i64,
}

#[derive(Serialize, Debug, ToResponse)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(rename = "registerTime")]
    pub registertime: DateTime<Utc>,
    #[serde(rename = "lastActivity")]
    pub lastactivity: DateTime<Utc>,
    pub invitor: Option<String>,
    pub upload: i64,
    pub download: i64,
    pub money: f64,
    pub rank: String,
    pub avatar: Option<String>,
    pub other: Option<serde_json::Value>,
    pub privacy: i32,
    pub email: String,
    pub passkey: String,
}

#[derive(Serialize, Debug, ToResponse)]
pub struct Invitation {
    pub sender: Option<String>,
    pub code: String,
    pub address: String,
    pub usage: bool,
}

#[derive(Deserialize, Serialize, Debug, ToResponse)]
pub struct Rank {
    pub id: i32,
    pub name: String,
    pub role: Vec<i16>,
    pub upload: i64,
    pub age: i64,
    pub next: Option<i32>,
}

#[derive(Serialize, Debug, ToResponse)]
pub struct Message {
    pub id: i64,
    pub sender: String,
    pub receiver: String,
    pub title: String,
    pub body: Option<String>,
    pub read: bool,
    #[serde(rename = "sendTime")]
    pub sendtime: DateTime<Utc>,
}