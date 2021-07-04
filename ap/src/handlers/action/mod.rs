use crate::{
    db::Db, auth::jwt, util::respond,
};
use actix_web::{
    Responder as Res,
    web::{self, Data, Path, ServiceConfig},
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .route("", web::get().to(get_all))
        .route("/{action_id}", web::get().to(get_by_id))
        ;
}

pub async fn get_all(_db: Data<Db>) -> impl Res {
    String::from("GET /action")
}
pub async fn get_by_id(_db: Data<Db>, id: Path<String>) -> impl Res {
    format!("GET /action/{}", &id)
}
