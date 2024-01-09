use crate::models::user::{login, register};
use actix_web::web;

pub fn user_route(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
}
