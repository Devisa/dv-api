use ap_com::{Model, Id};
use crate::{
    db::Db,
    util::respond,
    handlers::item::{
        get_all_item_fields,
        new_item_field,
        get_items_with_field,
        get_links_between_item_and_field,
    },
};
use ap_com::models::field::Field;
use sqlx::{prelude::*, postgres::Postgres};
use actix_web::{
    web::{HttpRequest,  Data, Json, Path, ServiceConfig, self}, Responder
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(new_field))
        )
        .service(web::scope("/field")
            .route("", web::get().to(get_all_field_field_links))
        )
        .service(web::scope("/item")
            .service(web::resource("")
                .route(web::get().to(get_all_item_fields))
                .route(web::post().to(new_item_field))
            )
        )
        .service(web::scope("/user/{user_id}")                         //        /field/user/3
            .service(web::resource("")                                 //
                .route(web::get().to(get_all_fields_user))             //        /field/user/3
            )                                                          //
            .service(web::resource("/value")                           //
                .route(web::get().to(get_all_field_values_user))       //        /field/user/3/value
            )                                                          //
            .service(web::resource("/target")                          //
                .route(web::get().to(get_all_field_targets_user))      //       /field/user/3/target
            )
        )
        .service(web::scope("/{field_id}").configure(individual_field_ops));
}

pub fn individual_field_ops(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_by_id))
            .route(web::post().to(update_by_id))
            .route(web::delete().to(delete_by_id))
        )
        .service(web::scope("/field")
            .service(web::resource("")
                .route(web::get().to(get_field_field_links))
                .route(web::post().to(link_new_field_to_field))
            )
            .service(web::resource("/{field_id}")
                .route(web::get().to(get_links_between_fields))
                .route(web::post().to(link_existing_field_to_field))
            )
        )
        .service(web::scope("/item")
            .route("", web::get().to(get_items_with_field))
            .route("/{item_id}", web::get().to(get_links_between_item_and_field))
        )
        .service(web::resource("/target")
            .route(web::get().to(get_field_targets))
            .route(web::get().to(new_field_target))
        )
        .service(web::resource("/value")
            .route(web::get().to(get_field_values))
            .route(web::get().to(new_field_value))
        );

}

pub async fn get_all(db: Data<Db>) -> impl Responder {
    match Field::get_all(&db.pool).await {
        Ok(fields) => respond::ok(fields),
        Err(e) => respond::err(e),
    }
}

pub async fn new_field(db: Data<Db>, field: Json<Field>) -> impl Responder {
    match field.into_inner().insert(&db.pool).await {
        Ok(field) => respond::created(field),
        Err(e) => respond::err(e)
    }
}


pub async fn get_by_id(db: Data<Db>, field_id: Path<Id>) -> impl Responder {
    match Field::get(&db.pool, field_id.into_inner()).await {
        Ok(Some(field)) => respond::found(field),
        Ok(None) => respond::not_found("COULD NOT FIND FIELD"),
        Err(e) => respond::err(e)
    }
}

pub async fn update_by_id(db: Data<Db>) -> impl Responder {
    "link ID".to_string()
}
pub async fn delete_by_id(db: Data<Db>, field_id: Path<Id>) -> impl Responder {
    match Field::delete(&db.pool, field_id.into_inner()).await {
        Ok(Some(field)) => respond::found(field),
        Ok(None) => respond::not_found("COULD NOT FIND FIELD"),
        Err(e) => respond::err(e)
    }
}

pub async fn add_item_field_link() -> impl Responder {
    "link ID".to_string()
}

pub async fn get_item_field_links() -> impl Responder {
    "link ID".to_string()
}

pub async fn get_all_fields_user() -> impl Responder {
    "link ID".to_string()
}

pub async fn get_all_field_values_user() -> impl Responder {
    "link ID".to_string()
}

pub async fn get_all_field_targets_user() -> impl Responder {
    "link ID".to_string()
}
pub async fn get_field_targets() -> impl Responder {
    "link ID".to_string()
}
pub async fn new_field_target() -> impl Responder {
    "link ID".to_string()
}
pub async fn get_field_values() -> impl Responder {
    "link ID".to_string()
}
pub async fn new_field_value() -> impl Responder {
    "link ID".to_string()
}

pub async fn get_field_field_links() -> impl Responder {
    "link ID".to_string()
}
pub async fn get_all_field_field_links() -> impl Responder {
    "link ID".to_string()
}

pub async fn new_field_field_link() -> impl Responder {
    "link ID".to_string()
}

pub async fn link_new_field_to_field() -> impl Responder {
    "link ID".to_string()
}
pub async fn link_existing_field_to_field() -> impl Responder {
    "link ID".to_string()
}

pub async fn get_links_between_fields() -> impl Responder {
    "link ID".to_string()
}

