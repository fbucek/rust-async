// Database
use diesel::prelude::*;

use anyhow::{anyhow, Result};
use diesel::dsl::*;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::schema::users::{self, dsl};
use super::{Conn, Pool};
use crate::utils::hash;

/// We literally never want to select `textsearchable_index_col`
/// so we provide this type and constant to pass to `.select`
type UserInfoSelect = (
    dsl::id,
    dsl::username,
    dsl::email,
    dsl::created_at,
    dsl::login_session,
);

pub const USER_INFO_COLUMNS: UserInfoSelect = (
    dsl::id,
    dsl::username,
    dsl::email,
    dsl::created_at,
    dsl::login_session,
);

/// When requesting login
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    /// Can be email or username
    pub username: String,
    pub password: String,
}

/// Response to /api/auth/login
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginInfo {
    pub username: String,
    pub login_session: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Queryable)]
struct User {
    #[serde(skip)]
    pub id: i32,
    pub username: String,
    #[serde(skip)]
    pub password: String,
    // pub first_name: String,
    // pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub login_session: String,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
struct NewUser<'a> {
    // pub first_name: &'a str,
    // pub last_name: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub login_session: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Queryable)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub login_session: String,
}

pub fn get_all_users(pool: Arc<Pool>) -> Result<Vec<UserInfo>> {
    let conn = pool.get().unwrap();
    // FIXME: exposing hash
    Ok(dsl::users
        .select(USER_INFO_COLUMNS)
        .load::<UserInfo>(&conn)?)
}

pub fn db_get_user_by_id(pool: Arc<Pool>, user_id: i32) -> Result<UserInfo> {
    let conn = pool.get().unwrap();
    Ok(dsl::users
        .find(user_id)
        .select(USER_INFO_COLUMNS)
        .get_result::<UserInfo>(&conn)?)
}

pub fn delete_single_user(db: Arc<Pool>, user_id: i32) -> Result<usize> {
    let conn = db.get().unwrap();
    let count = delete(dsl::users.find(user_id)).execute(&conn)?;
    if count < 1 {
        let text = format!("No user with id: {}", user_id);
        Err(anyhow!(text))
    } else {
        Ok(count)
    }
}

/// ## Steps
///
/// Signup will use `lowercase` for `username`
///
/// 1. Check if `username` exists -> return Error
/// 2. Create user with hashed passwor
/// 3. return created user ( TODO: is it necessary? )
pub fn signup_user(db: Arc<Pool>, user: &InputUser) -> Result<UserInfo> {
    log::info!("signup user");
    let conn = db.get().unwrap();

    // Use lower case for username only
    let username = user.username.to_lowercase();

    if dsl::users
        .filter(dsl::username.eq(&user.username))
        // .select(ALL_COLUMNS)
        .get_result::<User>(&conn)
        .is_err()
    {
        // Not storing password but HASH
        let hashed_password = hash::argon_hash(user.password.as_bytes())?;

        let new_user = NewUser {
            username: &username,
            password: &hashed_password,
            email: &user.email,
            created_at: chrono::Local::now().naive_local(),
            login_session: "",
        };
        diesel::insert_into(dsl::users)
            .values(&new_user)
            .execute(&conn)?;

        // Return last added user
        Ok(dsl::users
            .order(dsl::id.desc())
            .select(USER_INFO_COLUMNS)
            .get_result::<UserInfo>(&conn)?)
    } else {
        Err(anyhow!("User ( {} ) already present", user.username))
    }
}

/// ## Steps
///
/// 1. Check if `username` exists -> return Error
/// 2. Create user with hashed password ad
/// 3. return created user ( TODO: is it necessary? )
pub fn login_user(db: Arc<Pool>, login: &LoginRequest) -> Result<LoginInfo> {
    let conn = db.get().unwrap();
    // let conn = pool.get().unwrap();
    // Use lower case for username only
    let username = &login.username;
    // Get user based on LoginRequest
    let user_to_verify = dsl::users
        .filter(dsl::username.eq(username))
        .or_filter(dsl::email.eq(username))
        .get_result::<User>(&conn)?;

    // Check if password is not empty
    if user_to_verify.password.is_empty() {
        error!("Users password in database is empty");
        return Err(anyhow!("Users password in db is emtpy"));
    }
    if login.password.is_empty() {
        return Err(anyhow!("Entered password for login is empty"));
    }

    // Passwords are not empty -> validate hash
    argon2::verify_encoded(&user_to_verify.password, &login.password.as_bytes())?;

    // Generate login session
    // TODO: return previous session id when used before.
    let session_id = generate_login_uuid();

    let mut user_to_verify = user_to_verify;
    user_to_verify.login_session = session_id;
    // Store session_id in database
    update_user_login_session(&user_to_verify, &conn)?;

    Ok(LoginInfo {
        username: user_to_verify.username,
        login_session: user_to_verify.login_session,
    })
}

/// Invalidate UUID login_session for selected user
pub fn logout_user(db: Arc<Pool>, username: &str) -> Result<()> {
    let conn = db.get().unwrap();
    let mut user = dsl::users
        .filter(dsl::username.eq(username))
        .get_result::<User>(&conn)?;
    user.login_session = "".to_string();

    update_user_login_session(&user, &conn).map(|_| ())
}

/// Checks if UUID for login session is valid
pub fn is_valid_login_session(db: Arc<Pool>, user: &str, session_id: &str) -> bool {
    let conn = db.get().unwrap();
    dsl::users
        .filter(dsl::username.eq(user))
        .filter(dsl::login_session.eq(session_id))
        // .select(ALL_COLUMNS)
        .get_result::<User>(&conn)
        .is_ok()
}

/// Creates unique UUID identifier for login session
fn generate_login_uuid() -> String {
    uuid::Uuid::new_v4().to_simple().to_string()
}

/// Updates UUID login_session token into database
fn update_user_login_session(user: &User, conn: &Conn) -> Result<usize> {
    Ok(diesel::update(dsl::users.find(user.id))
        .set(dsl::login_session.eq(&user.login_session))
        .execute(conn)?)
}
