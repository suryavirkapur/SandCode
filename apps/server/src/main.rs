use actix_web::{web, App, HttpServer};
use routes::execute_code;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .route("/execute", web::post().to(execute_code))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
