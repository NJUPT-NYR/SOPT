use super::*;

type UserRet = Result<User, Error>;
type UserVecRet = Result<Vec<User>, Error>;
type SlimUserRet = Result<SlimUser, Error>;

/// A full user struct
/// 1. email, unique one
/// 2. username: unique one, at most 50 chars
/// 3. password: hashed password
/// 4. passkey: 32bit string
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

impl User {
    pub fn new(email: String, username: String, password: String, passkey: String) -> Self {
        User {
            id: 114514,
            email,
            username,
            password,
            passkey,
            role: 1,
        }
    }
}

/// Add new user to database, return the slim struct
pub async fn add_user(client: &sqlx::PgPool, user: User) -> SlimUserRet {
    Ok(sqlx::query_as!(
        SlimUser,
        "INSERT INTO users(email, username, password, passkey) \
        VALUES ($1, $2, $3, $4) RETURNING id, email, username, passkey, role;",
        user.email,
        user.username,
        user.password,
        user.passkey
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

/// Find user by email, for checking purpose
pub async fn find_user_by_email(client: &sqlx::PgPool, email: &str) -> UserVecRet {
    Ok(sqlx::query_as!(
        User,
        "SELECT * FROM users \
        WHERE email = $1;",
        email
        )
        .fetch_all(client)
        .await?)
}

/// update password, return the full one(for Debug use)
pub async fn update_password_by_username(client: &sqlx::PgPool, username: &str, new_pass: &str) -> UserRet {
    sqlx::query_as!(
        User,
        "UPDATE users SET password = $1 \
         WHERE username = $2 RETURNING *;",
        new_pass,
        username
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

/// update passkey, return the full one(for Debug use)
pub async fn update_passkey_by_username(client: &sqlx::PgPool, username: &str, new_key: &str) -> UserRet {
    sqlx::query_as!(
        User,
        "UPDATE users SET passkey = $1 \
         WHERE username = $2 RETURNING *;",
        new_key,
        username
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}