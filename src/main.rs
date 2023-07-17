mod api;
mod models;
mod repository;
use actix_web::{web, App, HttpServer,http::header};
use actix_cors::Cors;
use dotenv::dotenv;
use crate::api::endpoint::*;
struct AppState {
    app_name: String,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        let cors = Cors::permissive()
            .allowed_methods(vec!["GET", "POST","OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
        .wrap(cors)
        .service(get_otp_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
