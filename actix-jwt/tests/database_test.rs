
// Database
extern crate diesel;
use diesel::prelude::*; // SqliteConneciton
use diesel::r2d2;

#[macro_use]
extern crate diesel_migrations;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!("migrations");



#[cfg(test)]
mod tests {
    use super::*;
    use actixjwt::db;
    use actixjwt::db::schema::users::{self, dsl::*};
    use actixjwt::api;


    embed_migrations!("migrations");

    #[actix_rt::test]
    async fn test_get_user() {
        let test_db = nafta::sqlite::TestDb::new();
        let conn = test_db.conn()
            .expect("Not possible to get pooled connection");
        
        embedded_migrations::run(&conn)
            .expect("Migration not possible to run");

        let pool = std::sync::Arc::new(test_db.pool);

        // Test
        let all_user = db::users::get_all_users(pool);
        assert!(all_user.is_ok());
    }
}
