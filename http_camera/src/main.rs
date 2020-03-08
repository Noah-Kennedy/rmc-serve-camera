use actix_web::{HttpServer, web, App};
use camera_core::logging::{LOGGING_LEVEL, start_logging};
use actix_web::middleware::Logger;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    start_logging();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())

            .service(
            web::scope("/camera"),
        )
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}