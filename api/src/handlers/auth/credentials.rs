use std::sync::Arc;
use crate::{
    models::session::ApiSession,
    util::respond
};
use time::{Duration, OffsetDateTime};
use api_db::{Id, Db, Model};
use actix_web::{
    HttpRequest, HttpResponse, HttpResponseBuilder, Responder,
    http::header::{self, ContentType},
    cookie::Cookie,
    web::{self, ServiceConfig, Json, Data, Form, Path}
};
use api_common::{
    models::{
        Profile, Session,Account,
        credentials::{CredentialsSignup, CredentialsIn, Credentials},
        user::{User, UserIn}
    },
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::scope("/signup")
            .route("", web::post().to(signup))
            .route("/user", web::post().to(signup_user))
            .route("/creds", web::post().to(signup_creds))
            .route("/account", web::post().to(signup_account))
        )
        .route("", web::post().to(index))
        .route("/login", web::post().to(login_creds))
        .route("/logout", web::post().to(logout_creds))
        .route("/check", web::post().to(check_creds));

}

/// NOTE: Credentials signup -- four phases
///     1. User provides e-mail and real name (creates User row)
///     2. User provides username and password (creates Credentials row)
///     3. Credentials account (Devisa as the provider) created due to implication
///     4. User provides optional profile info (create Profile row -- can be empty)
/// The signup handler will handle steps 1 and 2, then pass on 3 to another handler.
///
/// (I know all the cloning is stupid and nooby af im still learning)
pub async fn signup(req: HttpRequest, db: Data<Db>, data: Form<CredentialsSignup>) -> actix_web::Result<HttpResponse> {
    let db = db.into_inner();
    let user = User::new(Some(data.clone().name), Some(data.clone().email), None)
        .insert(&db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error inserting user: {:?}", e);
            sentry::capture_error(&e);
            e
        }).expect("Could not insert user");
    println!("Created user. {:?}", &user.clone());
    let creds = Credentials::create(user.clone().id, data.clone().username, data.clone().password)
        .hash()
        .insert(&db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error inserting creds: {:?}", e);
            sentry::capture_error(&e);
            e
        }).expect("Could not insert creds");
    println!("Created creds. {:?}", &creds.clone());
    let acc = Account::new_devisa_creds_account(user.id.clone(),creds.id.clone());
    println!("ACCOUNT BEFORE INSERTION: {:?}", &acc.clone());
    acc.clone().insert(&db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error inserting account: {:?}", e);
            sentry::capture_error(&e);
            e
        }).expect("Could not insert account");
    println!("Created account {:?}", &acc.clone());
    let profile = Profile { user_id: user.clone().id,..Default::default() }
        .insert(&db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error inserting profile: {:?}", e);
            sentry::capture_error(&e);
            e
        }).expect("Could not insert profile");
    println!("Created profile. {:?}", &profile.clone());
    return Ok(HttpResponse::Ok()
        .json(creds)
        .with_header(("Content-Type", "application/json"))
        .respond_to(&req));
}

/// First 1/3 of signup
pub async fn signup_user(req: HttpRequest, db: Data<Db>, data: Form<CredentialsSignup>) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(""))
}

/// Second 1/3 of signup
pub async fn signup_creds(req: HttpRequest, db: Data<Db>, data: Form<CredentialsSignup>) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(""))
}

/// Final third of signup (creates Acct + Profile)
pub async fn signup_account(req: HttpRequest, db: Data<Db>, data: Form<CredentialsSignup>) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(""))
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
            let session = Session::create_two_day_session(&db.pool, creds.clone().user_id)
                .await
                .map_err(|e| {
                    tracing::info!("Could not create session! {}", e);
                    respond::err(e)
                })
                .unwrap_or_default();
            session.clone().set_access_token()
                .map_err(|e| {
                    tracing::info!("Err creating JWT: {:?}", e);
                    sentry::capture_error(&e.root_cause());
                    respond::err(e)
                })
                .expect("Could not generate access token / Could not set session access token");
            let _access_token = session.clone().access_token;
            match session.insert(&db.pool).await {
                Ok(sess) => {
                    let j = sess.access_token.clone();
                    let mut jwt_cookie = "dvsa-auth=".to_string();
                    jwt_cookie.extend(j.clone().get().chars());
                    return HttpResponse::Accepted()
                        .content_type(header::ContentType::json())
                        .insert_header(("dvsa-cred-auth",j.clone().get().as_str()))
                        .cookie(
                            Cookie::build("dvsa-cred-auth", &j.clone().get())
                                .path("/")
                                .secure(true)
                                .expires(OffsetDateTime::now_utc() + Duration::hours(48))
                                .domain("https://api.devisa.io")
                                .max_age(time::Duration::hours(48))
                                .http_only(false)
                                .same_site(actix_web::cookie::SameSite::Lax)
                                .finish()
                            )
                        .insert_header(("x-session-token", j.clone().get().as_str()))
                        .insert_header(("set-cookie", jwt_cookie.as_str()))
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

// TODO handle logout in in-memory session object
pub async fn logout_creds(sess: Data<ApiSession>, req: HttpRequest, db: Data<Db>, data: Form<CredentialsIn>) -> impl Responder {
    let mut _sess: Arc<ApiSession> = sess.into_inner();
    let _cookies = req.cookies().expect("Couild not load cookeis");
    if let Some(c) = req.cookie("dvsa-auth"){
        if let Some(mut sess_cookie) = req.cookie("dvsa-cred-auth") {
            sess_cookie.make_removal();
            tracing::info!("Logged out user successfully -- removed dvsa-auth and dvsa-cred-auth cookies for {}", data.username);
            return HttpResponse::Ok()
                .del_cookie(&sess_cookie)
                .body("Successfully logged out")
        }
        return HttpResponse::Ok().body("User has dvsa-auth, but not dvsa-cred-auth cookies. No user to log out")
    }
    HttpResponse::NotFound().body("No logged in user to log out")
}

pub async fn check_creds(sess: Data<ApiSession>, req: HttpRequest, db: Data<Db>) -> impl Responder {
    "".to_string()
}

pub async fn index(db: Data<Db>) -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use actix_http::StatusCode;
    use actix_web::{test::{TestRequest, self}, dev, web::{self, Form}};
    use api_common::models::{Account, Profile};
    use api_common::types::{Provider, ProviderType};

    fn new_creds_signup(username: &str, password: &str, email: &str, name: &str) -> CredentialsSignup {
        CredentialsSignup {
            username: username.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            name: name.to_string(),
        }
    }
    fn new_creds_in(username: &str, password: &str) -> CredentialsIn {
        CredentialsIn {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    #[actix_rt::test]
    async fn test_creds_login_ok() -> anyhow::Result<()> {
        Ok(())
    }

    #[actix_rt::test]
    async fn test_creds_logout_ok() -> anyhow::Result<()> {
        Ok(())
    }

    #[actix_rt::test]
    async fn test_creds_login_gives_jwt() -> anyhow::Result<()> {
        Ok(())
    }

    #[actix_rt::test]
    async fn test_creds_logout_removes_jwt() -> anyhow::Result<()> {
        Ok(())
    }

    #[actix_rt::test]
    async fn test_creds_signup_ok() -> anyhow::Result<()> {
        let db = db().await?;
        let creds_in = new_creds_signup("jerr_name", "jerr_name_pass", "jerr@email.com", "jerr");
        creds_in.clone().signup_credentials(&db.pool).await?;

        let req = TestRequest::get().uri("/auth/signup/creds")
            .set_json(&creds_in)
            .to_http_request();
        let resp = signup(req, Data::new(db.clone()), Form(creds_in)).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let user_out = User::get_by_username(&db.clone().pool, "username1").await?.unwrap();
        println!("Created user. {:?}", &user_out.clone());
        assert_eq!(user_out.clone().name.unwrap(), "user1".to_string());
        assert_eq!(user_out.clone().email.unwrap(), "user1@email.com".to_string());

        let creds_out = Credentials::get_by_user_id(&db.pool, user_out.clone().id).await?.unwrap();
        println!("Created creds. {:?}", &creds_out.clone());
        assert_eq!(creds_out.username, "username1".to_string());
        assert_eq!(creds_out.password, "pass1".to_string());
        assert_eq!(creds_out.user_id, user_out.clone().id);

        let acct_out = Account::get_by_provider_account_id(&db.clone().pool, creds_out.clone().id)
            .await?.unwrap();
        println!("Created account {:?}", &acct_out.clone());
        assert_eq!(acct_out.provider_type, "credentials".to_string());
        assert_eq!(acct_out.provider_id, Provider::Devisa);
        assert_eq!(acct_out.provider_account_id, creds_out.clone().id);
        assert_eq!(acct_out.user_id, user_out.clone().id);

        let prof_out = Profile::get_by_user_id(&db.pool, user_out.clone().id).await?.unwrap();
        println!("Created profile. {:?}", &prof_out.clone());
        assert_eq!(prof_out.user_id, user_out.clone().id);

        Profile::delete(&db.clone().pool, prof_out.clone().id).await?;
        Account::delete(&db.clone().pool, acct_out.clone().id).await?;
        Credentials::delete(&db.clone().pool, creds_out.clone().id).await?;
        User::delete(&db.clone().pool, user_out.clone().id).await?;
        Ok(())

    }
}
