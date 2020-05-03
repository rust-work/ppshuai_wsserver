
use actix_web::{web,HttpResponse};
use actix_files::Files;

use crate::ws::websocket::*;
use crate::http::test::*;
use crate::http::user::*;

pub fn access_routes(config: &mut web::ServiceConfig) {
    config
    // websocket
    .service(web::resource("/ws/").to(ws_route))
    // web api
    .service(get_user)
    .service(add_user)
    .service(web::resource("/api/login").route(web::post().to(login)))
    .service(web::resource("/api/logout").to(logout))
    .service(web::resource("/api").route(web::get().to(index)))
    .service(
        web::resource("/api/docs")
            .route(web::get().to(doc_index))
            .route(web::post().to(save_file))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    )
    .service(web::resource("/api/test").route(web::get().to(test)))
    // redirect to index.html
    //.service(Files::new("/", "./static/").index_file("index.html"))
    .service(web::resource("/").route(web::get().to(|| {
        HttpResponse::Found()
            .header("LOCATION", "/static/index.html")
            .finish()
    })))
    // static resources
    .service(Files::new("/static/", "static/"));
}