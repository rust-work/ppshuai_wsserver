
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::ws::session::WsChatSession;

pub async fn ws_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsChatSession::default(), &req, stream)
}
