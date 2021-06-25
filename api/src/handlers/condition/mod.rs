use actix::dev::MessageResponse;
use actix_web::{
    HttpRequest, HttpResponse, Responder, get, post, web::{self, ServiceConfig}, cookie,
};
use api_common::models::{Profile, account::AccountProvider, auth::CredentialsSignupIn, credentials::{CredentialsIn, Credentials}, user::{User, UserIn}};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        ;
}

/* pub fn index() -> impl Responder {
}
 */
