// Database
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!("migrations");

#[cfg(test)]
mod tests {
    use actixjwt::db;
    use actixjwt::db::models::InputUser;

    embed_migrations!("migrations");

    #[actix_rt::test]
    async fn test_get_user() {
        let test_db = nafta::sqlite::TestDb::new();
        let conn = test_db
            .conn()
            .expect("Not possible to get pooled connection");

        embedded_migrations::run(&conn).expect("Migration not possible to run");

        let pool = std::sync::Arc::new(test_db.pool);

        // Test
        let all_user = db::users::get_all_users(pool);
        assert!(all_user.is_ok());
    }

    #[actix_rt::test]
    async fn test_complex() {
        let test_db = nafta::sqlite::TestDb::new();
        let conn = test_db
            .conn()
            .expect("Not possible to get pooled connection");

        embedded_migrations::run(&conn).expect("Migration not possible to run");

        let pool = std::sync::Arc::new(test_db.pool);
        // Db must be empty
        let all_user = db::users::get_all_users(pool.clone())
            .expect("Not possible to get all users from database");
        assert!(all_user.is_empty());

        // ADD User
        let user = InputUser::new("John", "Doe", "john.doe@apple.com");
        db::users::add_single_user(pool.clone(), &user)
            .expect("Not possible to add new user");

        // GET Users
        let all_user = db::users::get_all_users(pool.clone())
            .expect("Not possible to get all users from database");
        assert_eq!(all_user.len(), 1);
        let dbuser =all_user.first().unwrap();
        assert_eq!(dbuser.first_name, user.first_name);
        assert_eq!(dbuser.last_name, user.last_name);
        assert_eq!(dbuser.email, user.email);

        // DELETE User
        let deleted_count = db::users::delete_single_user(pool.clone(), dbuser.id)
            .expect(&format!("Not possible to delete user with id: {}", dbuser.id));
        assert_eq!(deleted_count, 1, "Only one item should be deleted");

        // Database must be empty
        let all_user = db::users::get_all_users(pool.clone())
            .expect("Not possible to get all users from database");
        assert_eq!(all_user.len(), 0, "Database must be empty");
    }
}
