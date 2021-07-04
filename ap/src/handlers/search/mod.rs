use ap_com::{Model, Id, Db};
use actix_web::{
    HttpRequest,
    web::{self, ServiceConfig}, Responder
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("").route(web::get().to(search_index)));
}

async fn search_index(r: HttpRequest, db: web::Data<Db>) -> impl Responder {
    "Welcome".to_string()
}
