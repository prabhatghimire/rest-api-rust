use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;

mod api;
mod database;
mod models;

#[derive(Serialize)]
pub struct Response {
    pub status: i32,
    pub message: String,
}
// ...

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
        status: 200,
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
        status: 404,
    };
    Ok(HttpResponse::NotFound().json(response))
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(healthcheck)
//             .default_service(web::route().to(not_found))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todo_db = database::todo::Database::new();
    let app_data = web::Data::new(todo_db);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::todo::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
