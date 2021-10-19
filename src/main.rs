mod handlers;
mod scraper;
mod models;

use actix_web::{ App, HttpServer };


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handlers::root)
            .service(handlers::trends)
            .service(handlers::lang_trends)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
