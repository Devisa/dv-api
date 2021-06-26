use actix_web::http::header::ContentType;
use anyhow::Context;
use api_common::auth::jwt;
use api_common::auth::jwt::Role;
use api_common::models::{account::{Account, AccountProvider}, Session};
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
use api_common::{models::{Profile,  auth::CredentialsSignupIn, credentials::{CredentialsIn, Credentials}, user::{User, UserIn}}, types::Gender};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("").route(web::to(index)))
        .service(web::resource("/signup").route(web::get().to(signup_creds)))
        .service(web::resource("/login").route(web::get().to(login_creds)))
        .service(web::resource("/logout").route(web::get().to(logout_creds)));

}

/// NOTE: Credentials signup -- four phases
///     1. User provides e-mail and real name (creates User row)
///     2. User provides username and password (creates Credentials row)
///     3. Credentials account (Devisa as the provider) created due to implication
///     4. User provides optional profile info (create Profile row -- can be empty)
/// The signup handler will handle steps 1 and 2, then pass on 3 to another handler.
///
/// (I know all the cloning is stupid and nooby af im still learning)
pub async fn signup_creds(req: HttpRequest, db: Data<Db>, data: Form<CredentialsSignup>) -> actix_web::Result<HttpResponse> {
    let db = db.into_inner();
    let user = User::new(Some(data.clone().name), Some(data.clone().email), data.clone().image);
    tracing::info!("Created user.");
    let profile = Profile { user_id: user.id, ..Default::default() };
    tracing::info!("Created profile.");
    let creds = Credentials::create(user.id, data.clone().username, data.clone().password).hash();
    tracing::info!("Created creds.");
    let acc = AccountProvider::devisa_creds_account(
        user.id,
        creds.id,
        None,
        None,
        None
    );
    tracing::info!("Created user, creds, and account -- now inserting user....");
    let user = user.insert(&db.pool).await;

    tracing::info!("now inserting creds....");
    let creds = creds.insert(&db.pool).await;
    if let Ok(creds) = creds {
        tracing::info!("now inserting account....");
        let acct = acc.insert(&db.pool).await;

        tracing::info!("now inserting profile....");
        let profile = profile.insert(&db.pool).await;

        if user.is_ok() && acct.is_ok() && profile.is_ok() {
            return Ok(HttpResponse::Ok()
                .json(creds)
                .with_header(("Content-Type", "application/json"))
                .respond_to(&req));
        }
        return Ok(HttpResponse::BadRequest()
                .insert_header(("Content-Type", "application/json"))
                .body(format!(
                        "Internal error: Inserted creds, but one failure:\n
                         User insert: {}\n
                         Acct insert: {}\n
                         Profile insert: {}\n",
                         user.is_ok(), acct.is_ok(), profile.is_ok()
                 ))
                .respond_to(&req));
    }
    return Ok(HttpResponse::Forbidden().body("Invalid credentials"));
}

/// NOTE: Credentials login -- three phases
///     1. User provides e-mail and password (checks against Db user to authorize)
///     2. Create a new JWT for user, pass in Cookies and/or header
///     3. DB creates a new session row in the session table, use JWT as access key
/// The signup handler will handle steps 1 and 2, then pass on 3 to another handler.
pub async fn login_creds(req: HttpRequest, db: Data<Db>, data: Form<CredentialsIn>) -> impl Responder {
    let ver = Credentials::verify(&db.pool, data.username.as_str(), data.password.as_str());
    match ver.await {
        Ok(creds) => {
            tracing::info!("New user logged in! Username {:?}", data.username);
            let session = Session::create_two_day_session(&db.pool, creds.user_id)
                .await
                .map_err(|e| {
                    tracing::info!("Could not create session! {}", e);
                    respond::err(e)
                })
                .unwrap_or_default();
            let jwt = jwt::encode_token(creds.user_id, session.id, "dvsa-creds".into(), Role::User.to_string(), 48)
                .map_err(|e| {
                    tracing::info!("Err creating JWT: {:?}", e);
                    sentry::capture_error(&e.root_cause());
                    respond::err(e)
                })
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
                        .insert_header(("x-session-token", jwt.as_str()))
                        .insert_header(("set-cookie", jwt.as_str()))
                        .json(creds);
                },
                Err(e) => {
                    sentry::capture_error(&e);
                    tracing::info!("Could not insert session into db! {}", e);
                    return respond::err(e);
                }
            }
        },
        Err(e) => {
            sentry::capture_error(&e.root_cause());
            HttpResponse::NotFound()
                .content_type(ContentType::json())
                .body(format!("No user with tht username: {}", e))
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
