use actix_web::{
    Responder,
    web::{self, ServiceConfig}
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .route("", web::get().to(get_all));
}

pub fn user_routes(cfg: &mut ServiceConfig) {
    cfg;
}

pub async fn get_all() -> impl Responder {
    "".to_string()
}
