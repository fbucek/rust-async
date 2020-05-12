// Database
use diesel::prelude::*;
// use crate::diesel::QueryDsl;
// use crate::diesel::RunQueryDsl;
use diesel::dsl::*; //{delete, insert_into};
use std::sync::Arc;

use super::schema::users::{self, dsl::*};
use super::Pool;
use super::models::{User, InputUser};

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
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
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
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
