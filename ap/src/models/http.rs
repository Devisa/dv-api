use actix_web::cookie::{SameSite, Cookie};
use time::{OffsetDateTime, Duration};

/// Create a new cookie to insert for API token-based authentication
pub fn api_session_cookie<'a>(path: &'a str, name: &'a str, token: &'a str) -> Cookie<'a> {
    Cookie::build(name, token)
        .path(path)
        .secure(true)
        .expires(OffsetDateTime::now_utc() + Duration::hours(48))
        .domain("https://api.devisa.io")
        .max_age(Duration::hours(48))
        .http_only(false)
        .same_site(SameSite::Lax)
        .finish()
}
