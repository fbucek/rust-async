// Database
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!("migrations");

#[cfg(test)]
mod database {
    use actixjwt::db;
    use actixjwt::db::users::InputUser;

    embed_migrations!("migrations");

    #[actix_rt::test]
    async fn test_init_db_must_be_empty() {
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

        // Signup User
        let user = InputUser { 
            username: "johndoe".to_string(),
            password: "strong xxx".to_string(),
            email: "john.doe@apple.com".to_string(),
        };

        db::users::signup_user(pool.clone(), &user).expect("Not possible to add new user");

        // GET Users
        let all_user = db::users::get_all_users(pool.clone())
            .expect("Not possible to get all users from database");
        assert_eq!(all_user.len(), 1);
        let dbuser = all_user.first().unwrap();
        assert_eq!(dbuser.username, user.username);
        assert_eq!(dbuser.email, user.email);

        // DELETE User
        let deleted_count = db::users::delete_single_user(pool.clone(), dbuser.id).expect(
            &format!("Not possible to delete user with id: {}", dbuser.id),
        );
        assert_eq!(deleted_count, 1, "Only one item should be deleted");




        // DELETE non existins user
        let deleted_count = db::users::delete_single_user(pool.clone(), 1000).expect(
            &format!("Not possible to delete user with id: {}", dbuser.id),
        );
        assert_eq!(deleted_count, 0, "1000 does not exists anything must be deleted");



        // Database must be empty
        let all_user = db::users::get_all_users(pool.clone())
            .expect("Not possible to get all users from database");
        assert_eq!(all_user.len(), 0, "Database must be empty");
    }


    #[actix_rt::test]
    async fn test_auth() {
        let test_db = nafta::sqlite::TestDb::new();
        let conn = test_db
            .conn()
            .expect("Not possible to get pooled connection");

        embedded_migrations::run(&conn).expect("Migration not possible to run");

        let pool = std::sync::Arc::new(test_db.pool);

        // Signup User / new user
        let user = InputUser { 
            username: "johndoe".to_string(),
            password: "strong xxx".to_string(),
            email: "john.doe@apple.com".to_string(),
        };

        // Signup 
        db::users::signup_user(pool.clone(), &user).expect("Not possible to add new user");
        // Second signup must caused error
        assert!(db::users::signup_user(pool.clone(), &user).is_err());
    }
}
