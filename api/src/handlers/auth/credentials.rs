use api_common::auth::jwt;
use api_common::auth::jwt::Role;
use api_common::models::Session;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use actix_web::http::{HeaderName, header, HeaderValue};
use actix_web::cookie::{Cookie, CookieBuilder};
use api_db::{Db, Model};
use crate::util::respond;
use actix_web::{
    HttpRequest, HttpResponse, HttpResponseBuilder, Responder,
    web::{self, ServiceConfig, Json, Data, Form, Path}
};
use api_common::{models::{Profile, account::AccountProvider, auth::CredentialsSignupIn, credentials::{CredentialsIn, Credentials}, user::{User, UserIn}}, types::Gender};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("").route(web::to(index)))
        .service(web::resource("/signup").route(web::get().to(signup_creds)))
        .service(web::resource("/login").route(web::get().to(login_creds)))
        .service(web::resource("/logout").route(web::get().to(logout_creds)));

}

/// NOTE: Credentials signup -- three phases
///     1. User provides e-mail and real name (creates User row)
///     2. User provides username and password (creates Credentials row)
///     3. User provides optional profile info (create Profile row -- can be empty)
/// The signup handler will handle steps 1 and 2, then pass on 3 to another handler.
///
/// (I know all the cloning is stupid and nooby af im still learning)
pub async fn signup_creds(req: HttpRequest, db: Data<Db>, data: Form<CredentialsSignup>) -> impl Responder {
    let db = db.into_inner();
    let user = User::new(Some(data.clone().name), Some(data.clone().email), data.clone().image);
    let creds = Credentials::create(user.id, data.clone().username, data.clone().password).hash();
    let user = user.insert(&db.pool).await;
    let creds = creds.insert(&db.pool).await;
    if let Ok(creds) = creds {
        if user.is_ok() {
            return HttpResponse::Ok().json(creds)
                .with_header(("Content-Type", "application/json"))
                .respond_to(&req);
        }
        return HttpResponse::BadRequest()
                .with_header(("Content-Type", "application/json"))
                .respond_to(&req);
    }

    return HttpResponse::Forbidden().body("Invalid credentials");
}

/// NOTE: Credentials login -- three phases
///     1. User provides e-mail and password (checks against Db user to authorize)
///     2. Create a new JWT for user, pass in Cookies and/or header
///     3. DB creates a new session row in the session table, use JWT as access key
/// The signup handler will handle steps 1 and 2, then pass on 3 to another handler.
pub async fn login_creds(req: HttpRequest, db: Data<Db>, data: Form<CredentialsIn>) -> impl Responder {
    let hpw = Credentials::hash_password(data.password.as_str());
    match User::get_by_username(&db.pool, data.username.as_str()).await {
        Ok(Some(user)) => {
            tracing::info!("New user logged in! Username {:?}", data.username);
            let session = Session::create_two_day_session(&db.pool, user.id)
                .await
                .map_err(|e| {
                    tracing::info!("Could not create session! {}", e);
                    respond::err(e)
                })
                .unwrap_or_default();
            let jwt = jwt::encode_token(user.id, session.id, "dvsa-creds".into(), Role::User.to_string(), 48)
                .map_err(|e| {tracing::info!("Err creating JWT: {:?}", e); respond::err(e) })
                .unwrap_or_default();
            match session.set_access_token(jwt.to_string())
                .set_session_token(jwt.to_string())
                .insert(&db.pool).await {
                Ok(sess) => {
                    let j = jwt.clone();
                    return HttpResponse::Accepted()
                    .content_type(header::ContentType::json())
                    .insert_header(("dvsa-cred-auth",HeaderValue::from_static("jwt here")))
                    .cookie(
                        Cookie::build("dvsa-cred-auth", &jwt.to_string())
                            .path("/")
                            .secure(true)
                            .expires(OffsetDateTime::now())
                            .domain("https://api.devisa.io")
                            .max_age(time::Duration::hours(48))
                            .http_only(false)
                            .same_site(actix_web::cookie::SameSite::Lax)
                            .finish()
                        )
                    .json(user);
                },
                Err(e) => {
                    tracing::info!("Could not insert session into db! {}", e);
                    respond::err(e)
                }

            }
        },
        Ok(None) => {
            tracing::info!("Credentials login failed: No user with username {}", data.clone().username);
            respond::not_found("No user with that username")
        }
        Err(e) => {
            tracing::error!("CREDENTIALS LOGIN ERROR: {}", e);
            respond::err(e)
        }
    }

}

pub async fn logout_creds(req: HttpRequest, db: Data<Db>, data: Form<CredentialsIn>) -> impl Responder {
    "Hello".to_string()
}

pub async fn index(db: Data<Db>) -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CredentialsSignup {
    pub email: String,
    pub name: String,
    pub image: Option<String>,
    pub username: String,
    pub password: String,
    /* pub gender: Gender,
    pub birthdate: NaiveDate,
    pub country: String,
    pub language: String, */
}
impl CredentialsSignup {



}

pub struct CredUserSignup {
    pub email: String,
    pub name: String,

}
pub struct CredCredentialsSignup {
    pub username: String,
    pub password: String,
}
