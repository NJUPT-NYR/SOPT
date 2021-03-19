use super::*;

type UserVecRet = Result<Vec<User>, Error>;
type SlimUserRet = Result<SlimUser, Error>;
type SlimUserVecRet = Result<Vec<SlimUser>, Error>;

/// A full user struct
/// 1. email, unique one
/// 2. username: unique one, at most 50 chars
/// 3. password: hashed password
/// 4. passkey: 32bit string
/// 5. role: bit string for
/// management and permission
#[derive(Serialize, Debug, ToResponse)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub password: String,
    pub passkey: String,
    pub role: i64,
}

/// Slim one, mainly for security, hiding password
#[derive(Serialize, Debug, ToResponse)]
pub struct SlimUser {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub passkey: String,
    pub role: i64,
}

/// Add new user to database, return the slim struct
pub async fn add_user(client: &sqlx::PgPool, email: &str, username: &str, password: &str, passkey: &str) -> SlimUserRet {
    Ok(sqlx::query_as!(
        SlimUser,
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

/// Find user by username, return the full one
pub async fn find_user_by_username(client: &sqlx::PgPool, username: &str) -> UserVecRet {
    Ok(sqlx::query_as!(
        User,
        "SELECT * FROM users \
        WHERE username = $1;",
        username
        )
        .fetch_all(client)
        .await?)
}

/// Find user by username, return the full one
pub async fn find_user_by_username_slim(client: &sqlx::PgPool, username: &str) -> SlimUserRet {
    sqlx::query_as!(
        SlimUser,
        "SELECT id, email, username, passkey, role FROM users \
        WHERE username = $1;",
        username
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

/// Find user by email or username, for checking purpose
pub async fn check_existence(client: &sqlx::PgPool, email: &str, username: &str) -> Result<String, Error> {
    let ret: Vec<User> = sqlx::query_as!(
        User,
        "SELECT * FROM users \
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

/// update password
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

/// update regenerated passkey
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

/// give permissions to a user
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

/// delete permissions to a user
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

/// list all banned user
pub async fn list_banned_user(client: &sqlx::PgPool) -> SlimUserVecRet {
    Ok(sqlx::query_as!(
        SlimUser,
        "SELECT id, email, username, passkey, role FROM users \
        WHERE (role & 1) = 0;"
        )
        .fetch_all(client)
        .await?)
}