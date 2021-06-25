use std::{
    cell::{Ref, RefCell}, mem, rc::Rc,
    collections::HashMap,
};
use actix_web::{
    dev::{Extensions, Payload, RequestHead, ServiceRequest, ServiceResponse},
    Error, FromRequest, HttpMessage, HttpRequest,
};
use futures_util::future::{ok, Ready};
use serde::{de::DeserializeOwned, Serialize};

pub struct Session(Rc<RefCell<SessionInner>>);

pub trait UserSession {
    fn get_session(&self) -> Session;
}

impl UserSession for HttpRequest {
    fn get_session(&self) -> Session {
        Session::get_session(&mut *self.extensions_mut())
    }
}

impl UserSession for ServiceRequest {
    fn get_session(&self) -> Session {
        Session::get_session(&mut *self.extensions_mut())
    }
}

impl UserSession for RequestHead {
    fn get_session(&self) -> Session {
        Session::get_session(&mut *self.extensions_mut())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum SessionStatus {
    Changed,
    Purged,
    Renewed,
    Unchanged,
}

impl Default for SessionStatus {
    fn default() -> SessionStatus {
        SessionStatus::Unchanged
    }
}

#[derive(Default)]
struct SessionInner {
    state: HashMap<String, String>,
    status: SessionStatus,
}

impl Session {
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, Error> {
        if let Some(s) = self.0.borrow().state.get(key) {
            Ok(serde_json::from_str(s).ok())
        } else {
            Ok(None)
        }
    }

    pub fn entries(&self) -> Ref<'_, HashMap<String, String>> {
        Ref::map(self.0.borrow(), |inner| &inner.state)
    }

    pub fn insert(
        &self,
        key: impl Into<String>,
        value: impl Serialize,
    ) -> Result<(), Error> {
        let mut inner = self.0.borrow_mut();

        if inner.status != SessionStatus::Purged {
            inner.status = SessionStatus::Changed;
            if let Ok(v) = serde_json::to_string(&value) {
                inner.state.insert(key.into(), v);
            }
        }

        Ok(())
    }

    pub fn remove(&self, key: &str) -> Option<String> {
        let mut inner = self.0.borrow_mut();

        if inner.status != SessionStatus::Purged {
            inner.status = SessionStatus::Changed;
            return inner.state.remove(key);
        }

        None
    }

    pub fn remove_as<T: DeserializeOwned>(
        &self,
        key: &str,
    ) -> Option<Result<T, String>> {
        self.remove(key)
            .map(|val_str| match serde_json::from_str(&val_str) {
                Ok(val) => Ok(val),
                Err(_err) => {
                    log::debug!(
                        "removed value (key: {}) could not be deserialized as {}",
                        key,
                        std::any::type_name::<T>()
                    );
                    Err(val_str)
                }
            })
    }

    pub fn clear(&self) {
        let mut inner = self.0.borrow_mut();

        if inner.status != SessionStatus::Purged {
            inner.status = SessionStatus::Changed;
            inner.state.clear()
        }
    }

    pub fn purge(&self) {
        let mut inner = self.0.borrow_mut();
        inner.status = SessionStatus::Purged;
        inner.state.clear();
    }

    pub fn renew(&self) {
        let mut inner = self.0.borrow_mut();

        if inner.status != SessionStatus::Purged {
            inner.status = SessionStatus::Renewed;
        }
    }

    pub fn get_changes<B>(
        res: &mut ServiceResponse<B>,
    ) -> (SessionStatus, impl Iterator<Item = (String, String)>) {
        if let Some(s_impl) = res
            .request()
            .extensions()
            .get::<Rc<RefCell<SessionInner>>>()
        {
            let state = mem::take(&mut s_impl.borrow_mut().state);
            (s_impl.borrow().status.clone(), state.into_iter())
        } else {
            (SessionStatus::Unchanged, HashMap::new().into_iter())
        }
    }

    fn get_session(extensions: &mut Extensions) -> Session {
        if let Some(s_impl) = extensions.get::<Rc<RefCell<SessionInner>>>() {
            return Session(Rc::clone(&s_impl));
        }
        let inner = Rc::new(RefCell::new(SessionInner::default()));
        extensions.insert(inner.clone());
        Session(inner)
    }
}

impl FromRequest for Session {
    type Error = Error;
    type Future = Ready<Result<Session, Error>>;
    type Config = ();

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ok(Session::get_session(&mut *req.extensions_mut()))
    }
}
/* //! Cookie based sessions. See docs for [`CookieSession`].

use std::{collections::HashMap, rc::Rc};

use actix_web::cookie::{Cookie, CookieJar, Key, SameSite};
use actix_web::dev::{Service, Transform, ServiceRequest, ServiceResponse};
use actix_web::http::{header::SET_COOKIE, HeaderValue};
use actix_web::{Error, HttpMessage, ResponseError};
use derive_more::Display;
use futures_util::future::{ok, LocalBoxFuture, Ready};
use serde_json::error::Error as JsonError;
use time::{Duration, OffsetDateTime};

/// Errors that can occur during handling cookie session
#[derive(Debug, Display)]
pub enum CookieSessionError {
    /// Size of the serialized session is greater than 4000 bytes.
    #[display(fmt = "Size of the serialized session is greater than 4000 bytes.")]
    Overflow,

    /// Fail to serialize session.
    #[display(fmt = "Fail to serialize session")]
    Serialize(JsonError),
}

impl ResponseError for CookieSessionError {}

enum CookieSecurity {
    Signed,
    Private,
}

struct CookieSessionInner {
    key: Key,
    security: CookieSecurity,
    name: String,
    path: String,
    domain: Option<String>,
    lazy: bool,
    secure: bool,
    http_only: bool,
    max_age: Option<Duration>,
    expires_in: Option<Duration>,
    same_site: Option<SameSite>,
}

impl CookieSessionInner {
    fn new(key: &[u8], security: CookieSecurity) -> CookieSessionInner {
        CookieSessionInner {
            security,
            key: Key::derive_from(key),
            name: "actix-session".to_owned(),
            path: "/".to_owned(),
            domain: None,
            lazy: false,
            secure: true,
            http_only: true,
            max_age: None,
            expires_in: None,
            same_site: None,
        }
    }

    fn set_cookie<B>(
        &self,
        res: &mut ServiceResponse<B>,
        state: impl Iterator<Item = (String, String)>,
    ) -> Result<(), Error> {
        let state: HashMap<String, String> = state.collect();

        if self.lazy && state.is_empty() {
            return Ok(());
        }

        let value =
            serde_json::to_string(&state).map_err(CookieSessionError::Serialize)?;

        if value.len() > 4064 {
            return Err(CookieSessionError::Overflow.into());
        }

        let mut cookie = Cookie::new(self.name.clone(), value);
        cookie.set_path(self.path.clone());
        cookie.set_secure(self.secure);
        cookie.set_http_only(self.http_only);

        if let Some(ref domain) = self.domain {
            cookie.set_domain(domain.clone());
        }

        if let Some(expires_in) = self.expires_in {
            cookie.set_expires(OffsetDateTime::now_utc() + expires_in);
        }

        if let Some(max_age) = self.max_age {
            cookie.set_max_age(max_age);
        }

        if let Some(same_site) = self.same_site {
            cookie.set_same_site(same_site);
        }

        let mut jar = CookieJar::new();

        match self.security {
            CookieSecurity::Signed => jar.signed(&self.key).add(cookie),
            CookieSecurity::Private => jar.private(&self.key).add(cookie),
        }

        for cookie in jar.delta() {
            let val = HeaderValue::from_str(&cookie.encoded().to_string())?;
            res.headers_mut().append(SET_COOKIE, val);
        }

        Ok(())
    }

    /// invalidates session cookie
    fn remove_cookie<B>(&self, res: &mut ServiceResponse<B>) -> Result<(), Error> {
        let mut cookie = Cookie::named(self.name.clone());
        cookie.set_path(self.path.clone());
        cookie.set_value("");
        cookie.set_max_age(Duration::zero());
        cookie.set_expires(OffsetDateTime::now_utc() - Duration::days(365));

        let val = HeaderValue::from_str(&cookie.to_string())?;
        res.headers_mut().append(SET_COOKIE, val);

        Ok(())
    }

    fn load(&self, req: &ServiceRequest) -> (bool, HashMap<String, String>) {
        if let Ok(cookies) = req.cookies() {
            for cookie in cookies.iter() {
                if cookie.name() == self.name {
                    let mut jar = CookieJar::new();
                    jar.add_original(cookie.clone());

                    let cookie_opt = match self.security {
                        CookieSecurity::Signed => jar.signed(&self.key).get(&self.name),
                        CookieSecurity::Private => {
                            jar.private(&self.key).get(&self.name)
                        }
                    };

                    if let Some(cookie) = cookie_opt {
                        if let Ok(val) = serde_json::from_str(cookie.value()) {
                            return (false, val);
                        }
                    }
                }
            }
        }

        (true, HashMap::new())
    }
}
pub struct CookieSession(Rc<CookieSessionInner>);

impl CookieSession {
    pub fn signed(key: &[u8]) -> CookieSession {
        CookieSession(Rc::new(CookieSessionInner::new(
            key,
            CookieSecurity::Signed,
        )))
    }

    pub fn private(key: &[u8]) -> CookieSession {
        CookieSession(Rc::new(CookieSessionInner::new(
            key,
            CookieSecurity::Private,
        )))
    }

    pub fn path<S: Into<String>>(mut self, value: S) -> CookieSession {
        Rc::get_mut(&mut self.0).unwrap().path = value.into();
        self
    }

    pub fn name<S: Into<String>>(mut self, value: S) -> CookieSession {
        Rc::get_mut(&mut self.0).unwrap().name = value.into();
        self
    }

    pub fn domain<S: Into<String>>(mut self, value: S) -> CookieSession {
        Rc::get_mut(&mut self.0).unwrap().domain = Some(value.into());
        self
    }

    pub fn lazy(mut self, value: bool) -> CookieSession {
        Rc::get_mut(&mut self.0).unwrap().lazy = value;
        self
    }

    pub fn secure(mut self, value: bool) -> CookieSession {
        Rc::get_mut(&mut self.0).unwrap().secure = value;
        self
    }

    pub fn http_only(mut self, value: bool) -> CookieSession {
        Rc::get_mut(&mut self.0).unwrap().http_only = value;
        self
    }

    pub fn same_site(mut self, value: SameSite) -> CookieSession {
        Rc::get_mut(&mut self.0).unwrap().same_site = Some(value);
        self
    }

    pub fn max_age(self, seconds: i64) -> CookieSession {
        self.max_age_time(Duration::seconds(seconds))
    }

    pub fn max_age_time(mut self, value: time::Duration) -> CookieSession {
        Rc::get_mut(&mut self.0).unwrap().max_age = Some(value);
        self
    }

    pub fn expires_in(self, seconds: i64) -> CookieSession {
        self.expires_in_time(Duration::seconds(seconds))
    }

    pub fn expires_in_time(mut self, value: Duration) -> CookieSession {
        Rc::get_mut(&mut self.0).unwrap().expires_in = Some(value);
        self
    }
}

impl<S, B: 'static> Transform<S, ServiceRequest> for CookieSession
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>>,
    S::Future: 'static,
    S::Error: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type InitError = ();
    type Transform = CookieSessionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CookieSessionMiddleware {
            service,
            inner: self.0.clone(),
        })
    }
}

pub struct CookieSessionMiddleware<S> {
    service: S,
    inner: Rc<CookieSessionInner>,
}

impl<S, B: 'static> Service<ServiceRequest> for CookieSessionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>>,
    S::Future: 'static,
    S::Error: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let inner = self.inner.clone();
        let (is_new, state) = self.inner.load(&req);
        let prolong_expiration = self.inner.expires_in.is_some();
        Session::set_session(&mut req, state);

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            let res = match Session::get_changes(&mut res) {
                (SessionStatus::Changed, state) | (SessionStatus::Renewed, state) => {
                    res.checked_expr(|res| inner.set_cookie(res, state))
                }

                (SessionStatus::Unchanged, state) if prolong_expiration => {
                    res.checked_expr(|res| inner.set_cookie(res, state))
                }

                // set a new session cookie upon first request (new client)
                (SessionStatus::Unchanged, _) => {
                    if is_new {
                        let state: HashMap<String, String> = HashMap::new();
                        res.checked_expr(|res| inner.set_cookie(res, state.into_iter()))
                    } else {
                        res
                    }
                }

                (SessionStatus::Purged, _) => {
                    let _ = inner.remove_cookie(&mut res);
                    res
                }
            };

            Ok(res)
        })
    }
}
 */
