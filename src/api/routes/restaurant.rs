use crate::models::restaurant::get_popular_restaurant;
use actix_web::web;

pub fn restaurant_route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/restaurant/popular_restaurant").service(get_popular_restaurant));
}
