use crate::util::respond;
use api_common::models::link::Link;
use api_db::{Model, Id, Db};
use actix_web::{web::{Json, Path, Data, self, ServiceConfig}, Responder};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all_links))
            .route(web::post().to(new_abstract_link))
        )
        .service(web::scope("/{link_name}")
            .service(web::resource("")
                .route(web::get().to(get_or_create_key_link))
            )
            .service(web::resource("/{link_value}")
                .route(web::get().to(get_or_create_key_value_link))
            )
        )
        .service(web::resource("/{link_id}")
            .route(web::get().to(get_by_id))
        );
}

pub async fn new_abstract_link(db: Data<Db>, link: Json<Link>) -> impl Responder {
    match link.into_inner().insert(&db.pool).await {
        Ok(link) => respond::ok(link),
        Err(e) => respond::internal_error().body(format!("{}", e))
    }
}

pub async fn get_by_id(db: Data<Db>, link_id: Path<Id>) -> impl Responder {
    match Link::get(&db.pool, link_id.into_inner()).await {
        Ok(Some(link)) => respond::ok(link),
        Ok(None) => respond::not_found("No link with that id"),
        Err(e) => respond::internal_error().body(format!("{}", e))
    }

}
pub async fn get_or_create_key_link(db: Data<Db>, link_name: Path<String>) -> impl Responder {
    match Link::get_or_create_key_val(&db.pool, link_name.into_inner(), None).await {
        Ok(link) => respond::ok(link),
        Err(e) => respond::internal_error().body(format!("{}", e))
    }
}

pub async fn get_or_create_key_value_link(db: Data<Db>, link: Path<(String, String)>) -> impl Responder {
    let (name, value) = link.into_inner();
    match Link::get_or_create_key_val(&db.pool, name, Some(value)).await {
        Ok(link) => respond::ok(link),
        Err(e) => respond::internal_error().body(format!("{}", e))
    }
}

pub async fn get_all_links(db: Data<Db>, link: Json<Link>) -> impl Responder {
    match Link::get_all(&db.pool).await {
        Ok(links) => respond::ok(links),
        Err(e) => respond::internal_error().body(format!("{}", e))
    }

}
