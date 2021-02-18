pub mod user;

use crate::error::Error;
use tokio_postgres::Row;
use deadpool_postgres::Client;
use tokio_postgres::types::ToSql;

pub async fn exec_cmd_and_map<B, F>(
    client: &Client,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    f: F
) -> Result<Vec<B>, Error> where
    F: FnMut(&Row) -> B,
{
    let statement = client.prepare(query).await.unwrap();
    Ok(client.query(
        &statement,
        params
        ).await?
        .iter()
        .map(f)
        .collect())
}