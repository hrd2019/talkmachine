use actix_web::{middleware, web, App, HttpServer};

// #[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::new("\"%r\" %s %b %Dms"))
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}
