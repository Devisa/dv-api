pub mod credentials;

use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use crate::{
    db::Db, auth::jwt, util::respond,
};
use actix_web::{
    HttpRequest, HttpResponse, Responder, get, post,
    web::{self, ServiceConfig, Json, Data, Form, Path},
    cookie::{self, Cookie},
};
use api_common::models::{Model, Profile, account::AccountProvider, auth::CredentialsSignupIn, credentials::{CredentialsIn, Credentials}, user::{User, UserIn}};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("/jwt")
            .route(web::get().to(get_jwt))
            .route(web::post().to(refresh_jwt))
        )
        .service(web::scope("/login")
            .route("", web::post().to(login))
        )
        .service(web::scope("/logout")
            .route("", web::post().to(logout))
        )
        .service(web::scope("/check")
            .route("", web::get().to(check_auth))
        )
        .service(web::scope("/creds").configure(credentials::routes))
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


pub async fn check_auth(req: HttpRequest, ) -> actix_web::Result<HttpResponse> {
    if let Some(token) = req.cookie("dvsa-auth") {
        println!("{}", token.value());
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
                match User::get(&db.pool, claims.sub.parse::<Uuid>().unwrap()).await {
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
pub async fn logout(db: Data<Db>, req: HttpRequest) -> impl Responder {
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
pub async fn login(db: Data<Db>, creds: Json<CredentialsIn>) -> impl Responder {
    let creds = creds.into_inner();
    match Credentials::verify(&db.pool, &creds.username, &creds.password).await {
        Ok(creds) => {
            // Create session Session::Create(user_id, ...)
            let user = Credentials::get_user(&db.pool, creds.user_id).await.unwrap();
            let jwt = jwt::encode_token(creds.user_id, Uuid::nil(), "dvweb".into(), "user".to_string(), 72).unwrap();
            let exp = OffsetDateTime::now_utc() + Duration::days(3);
            HttpResponse::Ok()
                .append_header(("dvsa-auth", jwt.clone()))
                .cookie(cookie::Cookie::build("dvsa-auth", &jwt)
                    .same_site(cookie::SameSite::None)
                    .expires(exp)
                    .finish())
                .json(&user)
        },
        Err(e) => respond::unauthorized().body(&format!("UNAUTHORIZED Username or password incorrect {}", e)),
        _ => respond::unauthorized().body("UNAUTHORIZED. Username or password incorrect")
    }
}

pub async fn check_jwt(db: Data<Db>, req: HttpRequest) -> impl Responder {
    String::new()
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
    signup_req: Json<CredentialsSignupIn>,
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


pub async fn get_session_token(req: HttpRequest) -> Option<String> {
     req.headers()
         .get("next-auth.session-token")
         .map(|hv| hv.to_str()
             .map(|v| v.to_string())
             .ok()
         )
         .unwrap_or(None)
}
