use actix_web::web::Json;
use actix_web::{
    get, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use std::borrow::Borrow;
use talkmachine::{Chater, Message};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::new("\"%r\" %s %b %Dms"))
            .service(index)
            .route("/xx", web::post().to(xx))
            .service(pull)
            .service(push)
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}

#[get("/index.html")]
async fn index() -> impl Responder {
    // println!("{:#?}", msg);
    HttpResponse::Ok().body("asdfdasf")
}

async fn xx(req: HttpRequest, msg: web::Json<Message>) -> impl Responder {
    println!("REQ: {:#?}", req);
    HttpResponse::Ok().body(&msg.data)
}

#[post("/chat/pull")]
async fn pull(msg: Json<Message>) -> impl Responder {
    HttpResponse::Ok().body(&msg.data)
}

#[post("/chat/push")]
async fn push(msg: Json<Message>) -> impl Responder {
    HttpResponse::Ok().body(&msg.data)
}
