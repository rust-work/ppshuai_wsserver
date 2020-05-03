
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type SqliteDbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_sqlite_db_pool() -> SqliteDbPool {
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}