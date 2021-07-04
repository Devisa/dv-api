pub mod credentials;
pub mod api;
pub mod oauth;

use std::convert::TryFrom;
use time::{Duration, OffsetDateTime};
use ap_com::{Db, Model, Id};
use crate::{
    util::respond,
    models::session::ApiSession,
    error::{ApiError, ApiResult}
};
use actix_web::{
    HttpRequest, HttpResponse, Responder,
    web::{self, ServiceConfig, Json, Data},
    cookie::{self, Cookie},
};
use ap_com::{
    auth::jwt,
    models::{Account, Profile, Session,
        user::{User, UserIn, credentials::{CredentialsSignup, CredentialsIn, Credentials}},
    },
    types::{
        AccessToken, Expiration,
        token::Token,
    }
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .route("/login", web::to(login))
        .route("/logout", web::to(logout))
        .service(web::scope("/creds").configure(credentials::routes))
        .service(web::resource("/sess")
            .route(web::get().to(get_apisessions))
            .route(web::post().to(new_apisession))
            .route(web::delete().to(del_apisession))
        )
        .service(web::resource("/jwt")
            .route(web::get().to(get_jwt))
            .route(web::post().to(refresh_jwt))
        )
        .service(web::scope("/check")
            .route("", web::get().to(check_auth))
            .route("", web::post().to(check_token))
        )
        .service(web::scope("/token")
            .route("", web::get().to(get_session_token))
            )
        .service(web::scope("/signup")
            .route("", web::post().to(signup_full))
            // .route("/credentials", web::post().to(signup_credentials))
            .route("/user", web::post().to(signup_user))
            .route("/profile", web::post().to(signup_profile))
        );
}

pub async fn get_jwt(req: HttpRequest) -> HttpResponse {
    if let Some(token) = req.cookie("dvsa-auth") {
        HttpResponse::Ok().body(token.to_string())
    } else {
        HttpResponse::NotFound().body("No JWT token found")
    }
}

pub async fn refresh_jwt(req: HttpRequest) -> impl Responder {
    "Hello".to_string()
}


pub async fn check_auth(req: HttpRequest, sess: Data<ApiSession>) -> actix_web::Result<HttpResponse> {
    if let Some(token) = req.cookie("dvsa-auth") {
        tracing::info!("{}", token.value());
        match jwt::decode_token(token.value()) {
            Ok(claims) => Ok(respond::ok(claims)),
            Err(e) => Ok(HttpResponse::Unauthorized().body(&format!("Your token is expired or invalid, {}", e))),
        }
    } else {
        Ok(HttpResponse::Unauthorized()
            .body("Your token is missing"))
    }
}

pub async fn get_jwt_user(db: Data<Db>, req: HttpRequest, ) -> actix_web::Result<HttpResponse> {
    if let Some(token) = req.cookie("dvsa-auth") {
        match jwt::decode_token(token.value()) {
            Ok(claims) => {
                match User::get(&db.pool, Id::try_from(claims.sub).unwrap()).await {
                    Ok(Some(user)) => Ok(respond::ok(user)),
                    Ok(None) => Ok(respond::not_found("No user with that sub")),
                    Err(e) => Ok(respond::err(e)),
                }
            },
            Err(e) => Ok(HttpResponse::Unauthorized().body(&format!("Your token is expired or invalid, {}", e))),
        }
    } else {
        Ok(HttpResponse::Unauthorized()
            .body("Your token is missing"))
    }
}

/// POST /auth/logout : Endpoint to initiate entire end-to-end logout process
pub async fn logout(db: Data<Db>, sess: Data<ApiSession>, req: HttpRequest) -> impl Responder {
    let mut resp = HttpResponse::Ok();
    if let Some(token) = req.cookie("dvsa-auth") {
        if token.value() == "" {
            return resp.body("No user to log out");
        } else {

            resp.del_cookie(&token);
        }
        // TODO: Delete session associated with token
        resp.body("You have been logged out.")
    } else {
        resp.body("No user to log out!")
    }
}

/// POST /auth/login : Endpoint to initiate entire end-to-end login process
///     1. Check current access token (if available) ....
///     ...
/// TODO Note: Since login credentials require user_id input, user must provide some info to
///      fetch User corresponding to credentials. Allow Email and username login.
///      Email login: Get { email, password } -> fetch User -> fetch Creds -> check password
///      Username login: Get { username, password } -> fetch Creds -> check password -> fetch User
pub async fn login(db: Data<Db>, cr: Json<CredentialsIn>) -> ApiResult<impl Responder> {
    match Credentials::verify(&db.pool, &cr.username, &cr.password).await {
        Ok(creds) => {
            let user = Credentials::get_user(&db.pool, creds.clone().user_id)
                .await.expect("Could not fetch user from creds");
            let sess = Session::create(user.clone().id, Expiration::two_days())
                .expect("Could not create session");
            let acct = Account::get_by_provider_account_id(&db.pool, creds.clone().id)
                .await
                .expect("DB ERROR: Could not fetch account from prov. acct. ID")
                .expect("ERROR: No credentials provider account with that ID");
            let acct = acct.update_access_token(&db.pool, &sess.access_token)
                .await
                .expect("DB ERROR: Could not update account access token");
            let a_tok = sess.clone().access_token.get();
            Ok(HttpResponse::Ok()
                .append_header(("dvsa-auth", a_tok.as_str()))
                .cookie(Cookie::build("dvsa-auth", a_tok.as_str())
                    .same_site(cookie::SameSite::None)
                    .expires(
                        OffsetDateTime::now_utc() +
                        Duration::hours(sess.expires.hours_left() as i64)
                    )
                    .finish())
                .json(&user))
        },
        Err(e) => Ok(respond::unauthorized().body(&format!("UNAUTHORIZED Username or password incorrect {}", e))),
        _ => Ok(respond::unauthorized().body("UNAUTHORIZED. Username or password incorrect"))
    }
}

// TODO generate a new refresh/session token
pub async fn check_token(db: Data<Db>, req: HttpRequest) -> impl Responder {
    match req.cookie("dvsa-auth") {
        Some(token) => {
            let at = AccessToken::new(token.to_string());
            tracing::info!("CHECK_TOKEN: Found cookie access token {}", &at);
            // let header = req.headers().get("dvsa-auth")
            // tracing::info!("CHECK_TOKEN: Found header access token {}", ;
            let decoded = at.clone().decode()
                .expect("Could not decode JWT");
            tracing::info!("CHECK_TOKEN: Got decoded user {:?}", &decoded);
            if at.is_expired()
            .expect("Could not decode JWT") {
                HttpResponse::Unauthorized()
                    .insert_header(("dvsa-auth-valid", false.to_string()))
                    .json(&decoded)
            } else {
                HttpResponse::Ok()
                    .insert_header(("dvsa-auth-valid", true.to_string()))
                    .json(&decoded)
            }
        },
        None => {
            respond::not_found("No access token found in cookies")
        }
    }
}

/// POST /auth/signup : Endpoint to initiate entire end-to-end signup process
///     1. From name and email, create user row entry
///     2. From username and password, create credentials row entry
///     3. If not through OAUTH:
///         3a. Create corresponding accounts entry (credentials provider, if not through OAuth)
///         3b. Send verification email
///         3c. Create verification request row entry
///     4. From profile info (if applicable), create profile row entry
pub async fn signup_full(
    db: Data<Db>,
    req: HttpRequest,
    signup_req: Json<CredentialsSignup>,
) -> impl Responder {
    match signup_req.into_inner().signup_credentials(&db.pool).await {
        Ok(user) => respond::ok(user),
        Err(e) => respond::err(e),
    }
}

pub async fn signup_user(
    db: Data<Db>,
    req: HttpRequest,
    signup_req: Json<UserIn>,
) -> impl Responder {
    match signup_req.into_inner().insert(&db.pool).await {
        Ok(user) => respond::ok(user),
        Err(e) => respond::err(e),
    }
}

/* pub async fn signup_credentials(
    db: Data<Db>,
    req: HttpRequest,
    signup_req: Json<CredentialsIn>,
) -> impl Responder {
    let signup_req: CredentialsIn = signup_req.into_inner().hash();
    match signup_req.(&db.pool).await {
        Ok(creds) => {
            let acct = AccountProvider::devisa_account(
                creds.user_id, creds.id.unwrap_or_default()
            );
            match acct.insert(&db.pool).await {
                Ok(acct) => respond::ok(creds),
                Err(e) => respond::err(e),
            }
        }
        Err(e) => respond::err(e),
    }
} */

pub async fn signup_profile(
    db: Data<Db>,
    req: HttpRequest,
    signup_req: Json<Profile>,
) -> impl Responder {
    match signup_req.into_inner().insert(&db.pool).await {
        Ok(profile) => respond::ok(profile),
        Err(e) => respond::err(e),
    }
}


pub async fn get_session_token(req: HttpRequest, sess: Data<ApiSession>) -> Option<String> {
     req.headers()
         .get("next-auth.session-token")
         .map(|hv| hv.to_str()
             .map(|v| v.to_string())
             .ok()
         )
         .unwrap_or(None)
}

pub async fn get_apisessions(sess: Data<ApiSession>) -> impl Responder {

    "".to_string()
}
pub async fn new_apisession(sess: Data<ApiSession>) -> impl Responder {

    "".to_string()
}
pub async fn del_apisession(sess: Data<ApiSession>) -> impl Responder {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiResult;

    /// Stage 1 of the signup -- input user det., min email
    /// Stage 2 of the signup -- input credentials
    #[actix_rt::test]
    async fn check_creds_signup_ok() -> ApiResult<()> {
        let creds = UserIn {
            email:"testman1@email.com".into(),
            ..Default::default()
        };
        let creds = CredentialsIn {
            username: "testman1".into(),
            password: "testpass1".into()
        };
        Ok(())
    }

    #[actix_rt::test]
    async fn check_expired_token_creds_fail() -> ApiResult<()> {
        Ok(())
    }
}
