use std::fmt::{Debug, Display};
use actix_web::{BaseHttpResponse, HttpResponseBuilder, body::Body, dev::JsonBody, http::{StatusCode}, HttpResponse, web::{self, Json}};
use serde::{Serialize, Deserialize};

pub fn ok<T>(body: T) -> HttpResponse
where
    for<'a> T: Debug + PartialEq<T> + Serialize + Deserialize<'a>
{
    return HttpResponseBuilder::new(StatusCode::OK)
        .json(body);
}

pub fn found<T>(body: T) -> HttpResponse
where
    for<'a> T: Debug + PartialEq<T> + Serialize + Deserialize<'a>
{
    return HttpResponseBuilder::new(StatusCode::OK)
        .json(body);
}

pub fn created<T>(body: T) -> HttpResponse
where
    for<'a> T: Debug + PartialEq<T> + Serialize + Deserialize<'a>
{
    return HttpResponseBuilder::new(StatusCode::OK)
        .json(body);
}

pub fn err<E>(err: E) -> HttpResponse
where
    E: Display
{
    return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("ERROR: {}", err));
}

pub fn ok_msg(msg: &'static str) -> HttpResponse {
    return HttpResponse::Ok().body(msg);
}
pub fn err_msg(msg: &'static str) -> HttpResponse {
    return HttpResponse::InternalServerError().body(msg);
}

pub fn not_found(msg: &'static str) -> HttpResponse {
    return HttpResponse::NotFound()
        .body(&format!("NOT FOUND: {}", msg));
}

pub fn internal_error() -> HttpResponseBuilder {
    return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn not_implemented() -> HttpResponseBuilder {
    return HttpResponseBuilder::new(StatusCode::NOT_IMPLEMENTED)
}

pub fn not_modified(msg: &'static str) -> HttpResponse {
    return HttpResponse::NotModified()
        .body(msg)
}

pub fn unauthorized() -> HttpResponseBuilder {
    return HttpResponseBuilder::new(StatusCode::UNAUTHORIZED)
}

pub fn unavailable() -> HttpResponseBuilder {
    return HttpResponseBuilder::new(StatusCode::SERVICE_UNAVAILABLE)
}

pub fn bad_request() -> HttpResponseBuilder {
    return HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
}

pub fn forbidden() -> HttpResponseBuilder {
    return HttpResponseBuilder::new(StatusCode::FORBIDDEN)
}

pub fn accepted() -> HttpResponseBuilder {
    return HttpResponseBuilder::new(StatusCode::ACCEPTED)
}

pub fn gone(msg: &'static str) -> HttpResponse {
    return HttpResponseBuilder::new(StatusCode::GONE)
        .body(msg);
}
