use actix_web::{HttpResponse, Responder, Scope, web::{self, Json, Path, ServiceConfig}, web::Data};
use crate::{util::respond, Db, Model, Id};
use serde::{Serialize, Deserialize};

//TODO require responder trait bound to customize responses per model?
/// A trait which can be applied to structs implementing the Model trait,
///     which allows for CRUD routes to be built out rapidly without boilerplate.
#[async_trait::async_trait]
pub trait ModelRoutes
where
    for<'a> Self: 'static + Model + std::fmt::Debug + PartialEq + Serialize + Deserialize<'a>
{

    fn path() -> String {
        let mut path = String::new();
        path.push('/');
        path.push_str(Self::table().as_str());
        path.pop();
        path.to_string()
    }

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

    /// The service which encompasses all routes specified in the base trait and
    ///     in the model implementation of the model_routes() method
    fn service() -> Scope {
        web::scope(Self::path().as_str()).configure(Self::routes)
    }

    async fn service_get_all(db: Data<Db>) -> actix_web::Result<HttpResponse> {
        match Self::get_all(&db.pool).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_delete_all(db: Data<Db>) -> actix_web::Result<HttpResponse> {
        match Self::delete_all(&db.pool).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_delete_by_id(db: Data<Db>, id: Path<Id>) -> actix_web::Result<HttpResponse> {
        match Self::delete_by_id(&db.pool, id.into_inner()).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_add_new(db: Data<Db>, model: Json<Self>) -> actix_web::Result<HttpResponse> {
        match model.into_inner().insert(&db.pool).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_get_by_id(db: Data<Db>, id: Path<Id>) -> actix_web::Result<HttpResponse> {
        match Self::get(&db.pool, id.into_inner()).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    //TODO implement
    async fn service_update(db: Data<Db>) -> actix_web::Result<HttpResponse> {
        match Self::get_all(&db.pool).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
}
