use api_db::{Db, Model, Id};
use crate::{
    util::respond,
    handlers::record::{
        get_all_record_items,
        get_records_with_item,
        get_record_items,
        new_record_item,
    },
};
use api_common::models::{
    link::{Link, LinkedTo, Linked},
    item::{Item, ItemField},
    field::Field,
};
use sqlx::prelude::*;
use actix_web::{
    HttpResponse, get, post, delete,
    web::{Path, Data, HttpRequest, Json, ServiceConfig, self}, Responder
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .route("/stats", web::get().to(|| async { HttpResponse::Ok().body("GET /item/stats") }))
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(create_item))
            .route(web::delete().to(|| async { HttpResponse::Ok() }))
        )
        .service(web::resource("/item")
            .route(web::get().to(get_all_item_item_links))
        )
        .service(web::scope("/record")
            .route("/{record_id}", web::get().to(get_record_items))
            .service(web::resource("")
                .route(web::get().to(get_all_record_items))
                .route(web::post().to(new_record_item))
            )
        )
        .service(web::scope("/field")
            .route("/{field_id}", web::get().to(get_items_with_field))
            .service(web::resource("")
                .route(web::get().to(get_all_item_fields))
                .route(web::post().to(new_item_field))
            )
        )
        .service(web::scope("/{item_id}").configure(individual_item_ops));
}

pub fn individual_item_ops(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_by_id))
            .route(web::delete().to(delete_by_id))
            .route(web::post().to(update_by_id))
        )
        .service(web::scope("/item")
            .service(web::resource("")
                .route(web::get().to(get_item_item_links))
                .route(web::get().to(link_new_item_to_item))
            )
            .service(web::resource("/{item_id}")
                .route(web::post().to(get_links_between_items))
                .route(web::post().to(link_existing_item_to_item))
            )
        )
        .service(web::scope("/record")
            .service(web::resource("")
                .route(web::get().to(get_records_with_item))
                .route(web::post().to(add_record_item_link))
            )
        )
        .service(web::scope("/field")
            .service(web::resource("")
                .route(web::get().to(get_item_fields))
                .route(web::post().to(add_new_field))
            )
            .service(web::resource("/{field_id}")
                .route(web::post().to(add_existing_field))
                .route(web::get().to(get_links_between_item_and_field))
            )
        );
}

/// #[get("/")]
pub async fn get_all(db: Data<Db>) -> impl Responder {
    log::info!("Retrieving all items...");
    match Item::get_all(&db.pool).await {
        Ok(items) => respond::ok(items),
        Err(e) => respond::err(e)
    }
}

// #[post("/")]
pub async fn create_item(db: Data<Db>, item: Json<Item>) -> impl Responder {
    match item.into_inner().insert(&db.pool).await {
        Ok(items) => respond::created(items),
        Err(e) => respond::err(e),
    }
}

// #[get("/{item_id}")]
pub async fn get_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Item::get(&db.pool, id.clone()).await {
        Ok(Some(item)) => respond::found(item),
        Ok(None) => respond::not_found("NOT FOUND"),
        Err(e) => respond::err(e),
    }
}

// #[post("/{item_id}")]
pub async fn update_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Item::get(&db.pool, id.clone()).await {
        Ok(Some(item)) => respond::found(item),
        Ok(None) => respond::not_found("COULD NOT FIND"),
        Err(e) => respond::err(e),
    }
}

// #[delete("/{item_id}")]
pub async fn delete_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Item::delete(&db.pool, id.clone()).await {
        Ok(Some(item)) => respond::ok(item),
        Ok(None) => respond::not_found("NOT OFUND"),
        Err(e) => respond::err(e)
    }
}

// #[post("/item")]
pub async fn add_new_field(db: Data<Db>, item_id: Path<Id>, field: Json<Field>) -> impl Responder {
    match Item::get(&db.pool, item_id.clone()).await {
        Ok(Some(_item)) => {
            match field.into_inner().insert(&db.pool).await {
                Ok(field) => {
                    let item_field = ItemField::new_basic(item_id.into_inner(), field.id, None);
                    match item_field.insert(&db.pool).await {
                        Ok(item_field_link) => respond::ok(item_field_link),
                        Err(e) => respond::err(e)
                    }
                },
                Err(e) => respond::err(e)
            }
        },
        Ok(None) => respond::not_found(&"item NOT OFUND"),
        Err(e) => respond::err(e)
    }
}

// #[post("/item/{item_id}")]
pub async fn add_existing_field(db: Data<Db>, path: Path<(Id, Id)>, item_id: Json<Item>) -> impl Responder {
    let (item_id, field) = path.into_inner();
    match (Item::get(&db.pool, item_id.clone()).await, Item::get(&db.pool, item_id.clone()).await) {
        (Ok(Some(item)), Ok(Some(field))) => {
            match ItemField::new_basic(item.id, field.id, None).insert(&db.pool).await {
                Ok(item_field) => respond::ok(item_field),
                Err(e) => respond::err(e)
            }
        },
        (Ok(None), Ok(Some(_field))) => respond::not_found(&"NO field FOUND"),
        (Ok(Some(_item)), Ok(None)) => respond::not_found(&"NO ITEM FOUND"),
        (Ok(None), Ok(None)) => respond::not_found(&"NOT item OR ITEM FOUND"),
        (_, _) => respond::internal_error().body("ERROR")
    }
}

// #[get("/field")]
pub async fn get_item_fields(db: Data<Db>, item_id: Path<Id>) -> impl Responder {
    match Item::get(&db.pool, item_id.clone()).await {
        Ok(Some(item)) => {
            match <Item as LinkedTo<Field>>::get_links_to_entry(&db.pool, item_id.clone()).await {
                Ok(item_fields) => respond::ok(item_fields),
                Err(e) => respond::err(e)
            }
        },
        Ok(None) => respond::not_found(&"item NOT OFUND"),
        Err(e) => respond::err(e)
    }
}


// #[get("/field/{field_id}")]
pub async fn get_links_between_item_and_field(db: Data<Db>, path: Path<(Id, Id)>) -> impl Responder {
    let (item_id, field_id) = path.into_inner();
    match (Item::get(&db.pool, item_id.clone()).await, Field::get(&db.pool, field_id.clone()).await) {
        (Ok(Some(item)), Ok(Some(field))) => {
            match ItemField::linked_between(&db.pool, item.id, field.id).await {
                Ok(item_field_links) => respond::ok(item_field_links),
                Err(e) => respond::err(e)
            }
        },
        (Ok(None), Ok(Some(_field))) => respond::not_found(&"NO item FOUND"),
        (Ok(Some(_item)), Ok(None)) => respond::not_found(&"NO field FOUND"),
        (Ok(None), Ok(None)) => respond::not_found(&"NOT item OR ITEM FOUND"),
        (_, _) => respond::internal_error().body("ERROR")
    }
}

pub async fn add_item_item_link() -> impl Responder {
    "".to_string()
}

pub async fn get_all_item_fields() -> impl Responder {
    "".to_string()
}
pub async fn new_item_field() -> impl Responder {
    "".to_string()
}
pub async fn get_items_with_field() -> impl Responder {
    "".to_string()
}
pub async fn add_record_item_link() -> impl Responder {
    "".to_string()
}
pub async fn get_item_item_links() -> impl Responder {
    "".to_string()
}

pub async fn get_all_item_item_links() -> impl Responder {
    "".to_string()
}
pub async fn link_existing_item_to_item() -> impl Responder {
    "".to_string()
}
pub async fn link_new_item_to_item() -> impl Responder {
    "".to_string()
}
pub async fn get_links_between_items() -> impl Responder {
    "".to_string()
}
