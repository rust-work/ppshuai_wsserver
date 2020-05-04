

#[macro_use]
extern crate diesel;
use actix_cors::Cors;
use actix_web::{http::header, App, HttpServer};
use log::info;

mod db;
mod ws;
mod http;
mod routes;

use crate::db::mysql::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    dotenv::dotenv().ok();

    // set up database connection pool
    let pool = get_mysql_db_pool();
        
    let addr = "0.0.0.0:8080";

    let srv = HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(
                Cors::new()
                    .allowed_origin("http://10.0.2.239:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            // register resources
            .configure(routes::access_routes) 
    })
    .bind(&addr)?;

    info!("Starting http server: {}", &addr);

    srv.run().await
}
