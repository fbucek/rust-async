// Database
use diesel::prelude::*;

use diesel::dsl::*; 
use std::sync::Arc;
use anyhow::{anyhow, Result};


use serde::{Deserialize, Serialize};

use super::schema::users::{self, dsl};
use super::Pool;
use crate::utils::{hash, token::UserToken};


/// We literally never want to select `textsearchable_index_col`
/// so we provide this type and constant to pass to `.select`
type AllColumns = (
    dsl::id,
    dsl::username,
    dsl::email,
    dsl::login_session,
);

pub const ALL_COLUMNS: AllColumns = (
    dsl::id,
    dsl::username,
    dsl::email,
    dsl::login_session,
);


#[derive(Debug, PartialEq, Serialize, Deserialize, Queryable)]
pub struct User {
    // #[serde(skip)] 
    pub id: i32,
    pub username: String,
    // #[serde(skip)] 
    // pub password: String,
    // pub first_name: String,
    // pub last_name: String,
    pub email: String,
    // pub created_at: chrono::NaiveDateTime,
    pub login_session: String,
}


#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
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


pub fn get_all_users(pool: Arc<Pool>) -> Result<Vec<User>> {
    let conn = pool.get().unwrap();
    Ok(dsl::users
        .select(ALL_COLUMNS)
        .load::<User>(&conn)?
    )
}

pub fn db_get_user_by_id(pool: Arc<Pool>, user_id: i32) -> Result<User> {
    let conn = pool.get().unwrap();
    Ok(dsl::users
        .find(user_id)
        .select(ALL_COLUMNS)
        .get_result::<User>(&conn)?)
}


pub fn delete_single_user(db: Arc<Pool>, user_id: i32) -> Result<usize> {
    let conn = db.get().unwrap();
    let count = delete(dsl::users.find(user_id)).execute(&conn)?;
    Ok(count)
}


/// 
/// ## Steps 
/// 
/// 1. Check if `username` exists -> return Error
/// 2. Create user with hashed passwor
/// 3. return created user ( TODO: is it necessary? )
pub fn signup_user(db: Arc<Pool>, item: &InputUser) -> Result<User> {
    log::info!("Adding single user");
    let conn = db.get().unwrap();

    if dsl::users
        .filter(dsl::username.eq(&item.username))
        .select(ALL_COLUMNS)
        .get_result::<User>(&conn).is_err() 
    {
        info!("Implement adding to sqlite database");

        let hashed_password = hash::argon_hash(item.password.as_bytes())?;

        let new_user = NewUser {
            // first_name: &item.first_name,
            // last_name: &item.last_name,
            username: &item.username,
            password: &hashed_password,
            email: &item.email,
            created_at: chrono::Local::now().naive_local(),
            login_session: "",
        };
        diesel::insert_into(dsl::users).values(&new_user).execute(&conn)?;


        Ok(dsl::users
            .order(dsl::id.desc())
            .select(ALL_COLUMNS)
            // .limit(inserted_count as i64)
            .get_result::<User>(&conn)?)
    } else {
        Err(anyhow!("User alread present {}", item.username))
    }
}

pub fn is_valid_login_session(db: Arc<Pool>, user_token: &UserToken) -> bool {
    let conn = db.get().unwrap();
    dsl::users
        .filter(dsl::username.eq(&user_token.user))
        .filter(dsl::login_session.eq(&user_token.login_session))
        .select(ALL_COLUMNS)
        .get_result::<User>(&conn)
        .is_ok()
}


