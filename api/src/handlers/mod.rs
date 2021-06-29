pub mod search;
pub mod group;
pub mod graphql;
pub mod email;
pub mod post;
// pub mod rt;
pub mod book;
pub mod channel;
pub mod field;
pub mod message;
pub mod auth;
pub mod item;
pub mod link;
pub mod record;
pub mod topic;
pub mod user;
pub mod automata;
pub mod condition;
pub mod action;
pub mod ai;

//use async_graphql_actix_web::ServiceSchema;
use actix_web::{
    get, post, Responder,
    Scope, web::{self, HttpResponse, ServiceConfig, scope}
};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::{db::Db, util::respond};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(dict)
        .service(web::scope("/user").configure(user::routes))
        .service(web::scope("/item").configure(item::routes))
        .service(web::scope("/link").configure(link::routes))
        .service(web::scope("/auth").configure(auth::routes))
        .service(web::scope("/group").configure(group::routes))
        .service(web::scope("/record").configure(record::routes))
        .service(web::scope("/field").configure(field::routes))
        .service(web::scope("/post").configure(post::routes))
        .service(web::scope("/book").configure(book::routes))
        .service(web::scope("/channel").configure(channel::routes))
        .service(web::scope("/automata").configure(automata::routes))
        .service(web::scope("/action").configure(action::routes))
        .service(web::scope("/condition").configure(condition::routes))
        .service(web::scope("/ai").configure(ai::routes))
        .service(web::scope("/message").configure(message::routes))
        .service(web::scope("/email").configure(email::routes))
        // .service(web::scope("/rt").configure(rt::routes))
        .service(web::scope("/graphqlql").configure(graphql::routes))
        .service(web::scope("/topic").configure(topic::routes));
}


pub fn ad_hoc_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(dict);
}

#[get("/")]
pub async fn index() -> impl Responder {
    "Hi there!".to_string()
}

#[get("/dict")]
pub async fn dict(db: web::Data<Db>) -> impl Responder {
    "Hi there!".to_string()
    // match db.as_ref().create_basic_thesaurus().await {
    //     Ok(d) => respond::ok_msg("Enabled dict"),
    //     Err(e) => respond::err(e),
    // }
}

/* #[post("/echo")]
pub async fn echo(obj: web::Json<EchoObj>) -> impl Responder {
    let obj = obj.into_inner();
    return respond::ok(obj);
}

#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct EchoObj {
    pub data: String,
    pub time: DateTime<Utc>
}
 */
