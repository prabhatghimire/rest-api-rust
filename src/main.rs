use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger;
use serde::{Deserialize, Serialize};
use std::env;

mod api;
mod database;

mod models;
mod utils;

#[derive(Deserialize, Serialize)]
struct MyData {
    name: String,
}

#[derive(Serialize)]
struct Response {
    pub status: i32,
    pub message: String,
}

// // Define a handler that takes a parameter
// #[get("/")]
// async fn index(name: web::Path<String>) -> impl Responder {
//     // let db = database::mongodb_connector::get_database(); // get_database();
//     print!("${name}");
//     let res = Response {
//         message: "Everything is working fine".to_string(),
//         status: 200,
//     };
//     HttpResponse::Ok().json(res)
// }

#[get("/")]
async fn index() -> impl Responder {
    // let db = database::mongodb_connector::get_database(); // get_database();
    let res = Response {
        message: "Everything is working fine".to_string(),
        status: 200,
    };

    println!(":");
    HttpResponse::Ok().json(res)
}

async fn not_found() -> impl Responder {
    let response = Response {
        message: "Url not found".to_string(),
        status: 404,
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from the .env file
    dotenv().ok();
    // Access environment variables
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI not set in .env");
    // let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME not set in .env");
    let ip_addr = env::var("IP").expect("IP not set in .env");
    let port = env::var("PORT").expect("PORT not set in .env");

    let port_address = format!("{ip_addr}:{port}");
    println!("{} check reloading", port_address);

    env_logger::init();

    database::mongodb_connector::establish_connection(&mongodb_uri).await;
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/api").configure(api::routes::route_init))
            .service(index)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
