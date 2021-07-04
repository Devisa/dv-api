

use crate::{db::Db, util::respond};
use ap_com::models::group::Group;
use sqlx::{prelude::*, postgres::Postgres};
use actix_web::{
    http::StatusCode,
    delete, get, post, web::{HttpRequest,  ServiceConfig, self}, Responder
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg;
}
