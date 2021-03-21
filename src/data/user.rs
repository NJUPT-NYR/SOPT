use super::*;

pub async fn add_user(client: &sqlx::PgPool, email: &str, username: &str, password: &str, passkey: &str) -> AccountRet {
    Ok(sqlx::query_as!(
        Account,
        "INSERT INTO users(email, username, password, passkey) \
        VALUES ($1, $2, $3, $4) RETURNING id, email, username, passkey, role;",
        email,
        username,
        password,
        passkey
        )
        .fetch_one(client)
        .await?)
}

pub async fn find_user_by_username(client: &sqlx::PgPool, username: &str) -> AccountRet {
    sqlx::query_as!(
        Account,
        "SELECT id, email, username, passkey, role FROM users \
        WHERE username = $1;",
        username
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

pub async fn list_banned_user(client: &sqlx::PgPool) -> AccountVecRet {
    Ok(sqlx::query_as!(
        Account,
        "SELECT id, email, username, passkey, role FROM users \
        WHERE (role & 1) = 0;"
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_validation_by_name(client: &sqlx::PgPool, username: &str) -> ValidationVecRet {
    Ok(sqlx::query_as!(
        Validation,
        "SELECT id, username, password, role FROM users \
        WHERE username = $1;",
        username
        )
        .fetch_all(client)
        .await?)
}

/// find user by email or username, for checking purpose
pub async fn check_existence(client: &sqlx::PgPool, email: &str, username: &str) -> Result<String, Error> {
    let ret: Vec<Account> = sqlx::query_as!(
        Account,
        "SELECT id, email, username, passkey, role FROM users \
        WHERE email = $1 OR username = $2;",
        email,
        username
        )
        .fetch_all(client)
        .await?;

    if ret.is_empty() {
        Ok(String::new())
    } else if ret[0].username.eq(&username) {
        Ok(String::from("username"))
    } else {
        Ok(String::from("email"))
    }
}

pub async fn update_password_by_username(client: &sqlx::PgPool, username: &str, new_pass: &str) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE users SET password = $1 \
         WHERE username = $2;",
        new_pass,
        username
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn update_passkey_by_username(client: &sqlx::PgPool, username: &str, new_key: &str) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE users SET passkey = $1 \
         WHERE username = $2;",
        new_key,
        username
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn add_role_by_id(client: &sqlx::PgPool, id: i64, bit: i32) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE users SET role = role | (1::BIGINT << $1) \
        WHERE id = $2;",
        bit,
        id
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn delete_role_by_id(client: &sqlx::PgPool, id: i64, bit: i32) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE users SET role = role & ~(1::BIGINT << $1) \
        WHERE id = $2;",
        bit,
        id
        )
        .execute(client)
        .await?;

    Ok(())
}
