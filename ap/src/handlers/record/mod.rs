//! Record Handlers
//!
use crate::util::respond;
use ap_com::{Model, Db, Id, rel::link::{LinkedTo, Linked}};
use ap_com::models::{
    record::{Record, RecordItem},
    item::Item,
};
use actix_web::web::{Json, Data, Path,Form, HttpRequest, HttpResponse, ServiceConfig,  self};
use actix_web::Responder;

/// REPRESENTS RECORD BASE ROUTE /records
pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        // .service(get_all)
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(create_record))
        )
        .service(web::scope("/item")
            .service(web::resource("")
                .route(web::get().to(get_all_record_items))
                .route(web::post().to(new_record_item))
            )
            .service(web::resource("/{item_id}")
                .route(web::get().to(get_records_with_item))
                .route(web::post().to(new_record_with_link))
            )
        )
        .service(web::scope("/record")
            .service(web::resource("")
                .route(web::get().to(get_all_record_record_links))
            )
        )
        .service(web::scope("/{record_id}").configure(individual_record_ops));
        // .service(individual_record_ops())
}

/// REPRESENTS RECORD SUB-ROUTE /records/{record_id}
pub fn individual_record_ops(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_by_id))
            .route(web::delete().to(delete_by_id))
            .route(web::post().to(update_by_id))
        )
        .service(web::scope("/item")
            .service(web::resource("")
                .route(web::get().to(get_record_items))
                .route(web::post().to(add_new_item))
            )
            .service(web::resource("/{item_id}")
                .route(web::post().to(add_existing_item))
                .route(web::get().to(get_record_item_link))
            )
        )
        .service(web::scope("/user")
            .service(web::resource("")
                .route(web::get().to(get_all_user_record_links))
            )
            .service(web::resource("/{user_id}")
                .route(web::get().to(get_user_record_links))
                .route(web::post().to(add_new_user_record_link))              //         /record/3/user/4
            )
        )
        .service(web::scope("/group")
            .service(web::resource("")
                .route(web::get().to(get_all_group_record_links))
            )
            .service(web::resource("/{group_id}")
                .route(web::post().to(get_group_record_links))
                .route(web::post().to(add_new_group_record_link))
            )
        )
        .service(web::scope("/record")
            .service(web::resource("")
                .route(web::get().to(get_record_record_links))
                .route(web::post().to(link_new_record_to_record))
            )
            .service(web::resource("/{record_id}")
                .route(web::post().to(get_links_between_records))
                .route(web::post().to(link_existing_record_to_record))
            )
        );
}

// #[get("/")]
pub async fn get_all(db: Data<Db>) -> impl Responder {
    log::info!("Retrieving all records...");
    match Record::get_all(&db.pool).await {
        Ok(records) => respond::ok(records),
        Err(e) => respond::err(e),
    }
}

// #[post("/")]
pub async fn create_record(db: Data<Db>, record: Form<Record>) -> impl Responder {
    match record.into_inner().insert(&db.pool).await {
        Ok(records) => respond::created(records),
        Err(e) => respond::err(e)
    }
}

// #[get("/{record_id}")]
pub async fn get_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Record::get(&db.pool, id.clone()).await {
        Ok(Some(record)) => respond::found(record),
        Ok(None) => respond::not_found("COULD NOT FIND"),
        Err(e) => respond::err(e),
    }
}

// #[post("/{record_id}")]
pub async fn update_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Record::get(&db.pool, id.clone()).await {
        Ok(Some(record)) => respond::found(record),
        Ok(None) => respond::not_found("COULD NOT FIND"),
        Err(e) => respond::err(e),
    }
}

// #[delete("/{record_id}")]
pub async fn delete_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Record::delete(&db.pool, id.clone()).await {
        Ok(Some(record)) => respond::gone("DELETED RECORD"),
        Ok(None) => respond::not_found("COULD NOT FIND"),
        Err(e) => respond::err(e),
    }
}

// #[post("/item")]
pub async fn add_new_item(db: Data<Db>, record_id: Path<Id>, item: Json<Item>) -> impl Responder {
    match Record::get(&db.pool, record_id.clone()).await {
        Ok(Some(rec)) => {
            match item.into_inner().insert(&db.pool).await {
                Ok(item) => {
                    let record_item = RecordItem::new_basic(record_id.into_inner(), item.id, None);
                    match record_item.insert(&db.pool).await {
                        Ok(record_item) => respond::ok(record_item),
                        Err(e) => respond::err(e),
                    }
                },
                Err(e) => respond::err(e),
            }
        },
        Ok(None) => respond::not_found("RECORD NOT OFUND"),
        Err(e) => respond::err(e),
    }
}

// #[post("/item/{item_id}")]
pub async fn add_existing_item(db: Data<Db>, path: Path<(Id, Id)>, item_id: Json<Item>) -> impl Responder {
    let (record_id, item_id) = path.into_inner();
    match (Record::get(&db.pool, record_id.clone()).await, Item::get(&db.pool, item_id.clone()).await) {
        (Ok(Some(record)), Ok(Some(item))) => {
            let record_item = RecordItem::new_basic(record.id, item.id, None);
            match record_item.insert(&db.pool).await {
                Ok(record_item) => respond::ok(record_item),
                Err(e) => respond::err(e),
            }
        },
        (Ok(None), Ok(Some(_item))) => respond::not_found("NO RECORD FOUND"),
        (Ok(Some(_record)), Ok(None)) => respond::not_found("NO ITEM FOUND"),
        (Ok(None), Ok(None)) => respond::not_found("NOT RECORD OR ITEM FOUND"),
        (_, _) => respond::internal_error().body("ERROR")
    }
}

// #[get("/item")]
pub async fn get_record_items(db: Data<Db>, record_id: Path<Id>) -> impl Responder {
    match Record::get(&db.pool, record_id.clone()).await {
        Ok(Some(rec)) => {
            match <Record as LinkedTo<Item>>::get_links_to_entry(&db.pool, record_id.clone()).await {
                Ok(items) => respond::ok(items),
                Err(e) => respond::err(e),
            }
        },
        Ok(None) => respond::not_found("RECORD NOT OFUND"),
        Err(e) => respond::err(e)
    }
}


// #[get("/item/{item_id}")]
pub async fn get_record_item_link(db: Data<Db>, path: Path<(Id, Id)>) -> impl Responder {
    let (record_id, item_id) = path.into_inner();
    match (Record::get(&db.pool, record_id.clone()).await, Item::get(&db.pool, item_id.clone()).await) {
        (Ok(Some(record)), Ok(Some(item))) => {
            match RecordItem::linked_between(&db.pool, record.id, item.id).await {
                Ok(record_item_links) => respond::ok(record_item_links),
                Err(e) => respond::err(e),
            }
        },
        (Ok(None), Ok(Some(item))) => respond::not_found("NO RECORD FOUND"),
        (Ok(Some(record)), Ok(None)) => respond::not_found("NO ITEM FOUND"),
        (Ok(None), Ok(None)) => respond::not_found("NOT RECORD OR ITEM FOUND"),
        (_, _) => respond::internal_error().finish(),
    }
}

pub async fn get_all_record_items(db: Data<Db>) -> impl Responder {
    match RecordItem::get_all(&db.pool).await {
        Ok(record_items) => respond::ok(record_items),
        Err(e) => respond::err(e),
    }
}

pub async fn new_record_item(db: Data<Db>, record_item: Json<RecordItem>) -> impl Responder {
    match record_item.into_inner().insert(&db.pool).await {
        Ok(record_items) => respond::ok(record_items),
        Err(e) => respond::err(e),
    }
}


pub async fn get_records_with_item(db: Data<Db>, item_id: Path<Id>) -> impl Responder {
    match <Record as LinkedTo<Item>>::get_entries_linked_to(&db.pool, item_id.into_inner()).await {
        Ok(records) => respond::ok(records),
        Err(e) => respond::err(e),
    }
}

pub async fn new_record_with_link() -> impl Responder {
    "".to_string()
}
pub async fn get_all_group_record_links() -> impl Responder {
    "".to_string()
}
pub async fn get_all_user_record_links() -> impl Responder {
    "".to_string()
}
pub async fn add_new_user_record_link() -> impl Responder {
    "".to_string()
}
pub async fn add_new_group_record_link() -> impl Responder {
    "".to_string()
}
pub async fn get_group_record_links() -> impl Responder {
    "".to_string()
}
pub async fn get_user_record_links() -> impl Responder {
    "".to_string()
}
pub async fn get_all_record_record_links() -> impl Responder {
    "".to_string()
}
pub async fn get_record_record_links() -> impl Responder {
    "".to_string()
}
pub async fn link_existing_record_to_record() -> impl Responder {
    "".to_string()
}
pub async fn link_new_record_to_record() -> impl Responder {
    "".to_string()
}
pub async fn get_links_between_records() -> impl Responder {
    "".to_string()
}
