use crate::Response;
use actix_web::{get, HttpResponse, Responder};

// use crate::database;

#[get("/")]
async fn get_popular_restaurant() -> impl Responder {
    // let db = database::mongodb_connector::get_database(); // get_database();
    let res = Response {
        message: "Everything is working fine".to_string(),
        status: 200,
    };
    HttpResponse::Ok().json(res)
}
