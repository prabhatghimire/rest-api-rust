pub mod routes {
    use actix_web::web;

    mod restaurant;
    mod user;

    pub fn route_init(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/user").configure(user::user_route))
            .service(web::scope("/restaurant").configure(restaurant::restaurant_route));
    }
}
