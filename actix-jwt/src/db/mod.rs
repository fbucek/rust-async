use diesel::prelude::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};

// Reexport
pub use diesel::prelude::*;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub mod models;
pub mod schema;
pub mod users;
