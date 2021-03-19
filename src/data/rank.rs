use super::*;

type RankRet = Result<Rank, Error>;
type RankVecRet = Result<Vec<Rank>, Error>;

#[derive(Deserialize, Serialize, Debug, ToResponse)]
pub struct Rank {
    pub id: i32,
    pub name: String,
    pub role: Vec<i16>,
    pub upload: Option<i64>,
    pub age: Option<i64>,
    pub available: bool,
    pub next: Option<i32>,
}

pub async fn update_or_add_rank(client: &sqlx::PgPool, rank: Rank) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO rank(name, role, upload, age, available, next) \
        VALUES($1, $2, $3, $4, $5, $6) ON CONFLICT (name) DO \
        UPDATE SET name = $1, role = $2, upload = $3, age = $4, available = $5, next = $6;",
        rank.name,
        &rank.role,
        rank.upload,
        rank.age,
        rank.available,
        rank.next
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn find_rank_by_username(client: &sqlx::PgPool, username: &str) -> RankRet {
    sqlx::query_as!(
        Rank,
        "SELECT rank.id, name, role, rank.upload, age, available, next FROM rank \
        INNER JOIN user_info ON rank.name = user_info.rank \
        WHERE user_info.username = $1;",
        username
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

pub async fn find_rank_by_id(client: &sqlx::PgPool, id: i32) -> RankRet {
    sqlx::query_as!(
        Rank,
        "SELECT * FROM rank \
        WHERE id = $1;",
        id
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

pub async fn find_all_ranks(client: &sqlx::PgPool) -> RankVecRet {
    Ok(sqlx::query_as!(
        Rank,
        "SELECT * FROM rank;"
        )
        .fetch_all(client)
        .await?)
}