pub mod session;
pub mod http;
pub mod user;
pub mod credentials;
pub mod rpc;

use actix_web::{Responder, web::{self, Json, Path, ServiceConfig},  web::Data, HttpResponse};
use session::{ ApiSession, SessionInfo, SessionIn };
use ap_com::{Db, Model, Id};
use crate::util::respond;
use serde::{Serialize, Deserialize};

use crate::ApiResult;

//TODO require responder trait bound to customize responses per model?
/// A trait which can be applied to structs implementing the Model trait,
///     which allows for CRUD routes to be built out rapidly without boilerplate.
#[async_trait::async_trait]
pub trait ModelRoutes
where
    for<'a> Self: 'static + Model + std::fmt::Debug + PartialEq + Serialize + Deserialize<'a>
{

    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .service(web::scope("").configure(Self::model_routes))
            .service(web::resource("")
                .route(web::get().to(Self::service_get_all))
                .route(web::post().to(Self::service_add_new))
                .route(web::delete().to(Self::service_delete_all))
                .route(web::put().to(Self::service_update))
            )
            .service(web::resource("/id/{id}")
                .route(web::get().to(Self::service_get_by_id))
                .route(web::delete().to(Self::service_delete_by_id))
            );
    }

    fn model_routes(cfg: &mut ServiceConfig);

    async fn service_get_all(db: Data<Db>) -> ApiResult<HttpResponse> {
        match Self::get_all(&db.pool).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_delete_all(db: Data<Db>) -> ApiResult<HttpResponse> {
        match Self::delete_all(&db.pool).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_delete_by_id(db: Data<Db>, id: Path<Id>) -> ApiResult<HttpResponse> {
        match Self::delete_by_id(&db.pool, id.into_inner()).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_add_new(db: Data<Db>, model: Json<Self>) -> ApiResult<HttpResponse> {
        match model.into_inner().insert(&db.pool).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_get_by_id(db: Data<Db>, id: Path<Id>) -> ApiResult<HttpResponse> {
        match Self::get(&db.pool, id.into_inner()).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    //TODO implement
    async fn service_update(db: Data<Db>) -> ApiResult<HttpResponse> {
        match Self::get_all(&db.pool).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
}
