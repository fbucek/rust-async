use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::SqliteConnection;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
