use actix_web::web::Json;
use actix_web::{
    get, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use std::borrow::Borrow;
use talkmachine::Message;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::new("\"%r\" %s %b %Dms"))
            .service(no_params)
            .service(index)
            .service(xxx)
            .route("/xx", web::post().to(xx))
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

async fn xx(req: HttpRequest, _info: web::Json<Message>) -> impl Responder {
    println!("REQ: {:#?}", req);
    HttpResponse::Ok().body("asdfdasf")
}

#[post("/xxx/xx")]
async fn xxx(msg: Json<Message>) -> impl Responder {
    HttpResponse::Ok().body(&msg.data)
}

#[get("/")]
async fn no_params() -> &'static str {
    "Hello world!\r\n"
}
