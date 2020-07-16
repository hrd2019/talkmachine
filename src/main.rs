use crate::chat::chat::Chat;
use actix_web::web::Json;
use actix_web::{
    get, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use std::borrow::Borrow;
use talkmachine::{Chater, Message};

mod chat;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let c = Chat::new();
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::new("\"%r\" %s %b %Dms"))
            .service(index)
            .service(pull)
            .route("/push", web::post().to(c.push))
            .route("/pull", web::post().to(c.pull))
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}

#[get("/index")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[post("/chat/pull")]
async fn pull(msg: Json<Message>) -> impl Responder {
    HttpResponse::Ok().body(&msg.data)
}
