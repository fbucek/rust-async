use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub login_session: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl InputUser {
    pub fn new<FN, LN, E>(first_name: FN, last_name: LN, email: E) -> Self
    where
        FN: Into<String>,
        LN: Into<String>,
        E: Into<String>,
    {
        InputUser {
            first_name: first_name.into(),
            last_name: last_name.into(),
            email: email.into(),
        }
    }
}
