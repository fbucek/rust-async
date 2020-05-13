// Database
use diesel::prelude::*;

use diesel::dsl::*; 
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::schema::users::{self, dsl::*};
use super::Pool;
use crate::models::token::UserToken;

#[derive(Debug, PartialEq, Serialize, Deserialize, Queryable)]
pub struct User {
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


pub fn get_all_users(pool: Arc<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    Ok(users.load::<User>(&conn)?)
}

pub fn db_get_user_by_id(pool: Arc<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}

pub fn add_single_user(db: Arc<Pool>, item: &InputUser) -> Result<User, diesel::result::Error> {
    log::info!("Adding single user");
    let conn = db.get().unwrap();
    // Struct with user
    let new_user = NewUser {
        // first_name: &item.first_name,
        // last_name: &item.last_name,
        username: &item.username,
        password: &item.password,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
        login_session: "",
    };
    let _inserted_count = insert_into(users).values(&new_user).execute(&conn)?;

    Ok(users
        .order(id.desc())
        // .limit(inserted_count as i64)
        .get_result::<User>(&conn)?)
}

pub fn delete_single_user(db: Arc<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}

pub fn is_valid_login_session(db: Arc<Pool>, user_token: &UserToken) -> bool {
    let conn = db.get().unwrap();
    users
        .filter(username.eq(&user_token.user))
        .filter(login_session.eq(&user_token.login_session))
        .get_result::<User>(&conn)
        .is_ok()
}
