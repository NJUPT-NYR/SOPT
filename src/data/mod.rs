pub mod user;
pub mod invitation;
// pub mod torrent;

use sqlx::{Pool, postgres::Postgres};

type PgPool = Pool<Postgres>;