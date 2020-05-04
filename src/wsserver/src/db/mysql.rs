

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type MySqlDbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn get_mysql_db_pool() -> MySqlDbPool {
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}