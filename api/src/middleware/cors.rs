use std::{convert::TryInto,  rc::Rc};
use std::{collections::HashSet, convert::TryFrom, fmt, };
use actix::Response;
use actix_http::body::{AnyBody, MessageBody };
use actix_web::HttpResponseBuilder;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    error::{Error, Result},
    http::{
        header::{self, HeaderValue},
        Method,
    },
    HttpResponse,
};
use futures::poll;
use futures_util::future::{ok, Either, FutureExt as _, LocalBoxFuture, Ready};
use log::debug;

use inner::Inner;

pub fn cors_middleware() -> self::builder::Cors {
    builder::Cors::permissive()
        .supports_credentials()
        .allowed_origin("http://localhost:3000")
        .allowed_origin("https://devisa.io")
        .allowed_origin("http://localhost:3001")
        .allowed_origin("https://wx.devisa.io")
        .allowed_origin("https://io.devisa.io")
        .allowed_origin("https://app.devisa.io")
        .allowed_origin("https://ap.devisa.io")
        .allowed_origin("https://dv.devisa.io")
        .allowed_origin("https://dev.devisa.io")
        .allowed_origin("https://dvsa.io")
        .allow_any_header()
        .allow_any_origin()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllOrSome<T> {
    /// Everything is allowed. Usually equivalent to the `*` value.
    All,

    /// Only some of `T` is allowed
    Some(T),
}

/// Default as `AllOrSome::All`.
impl<T> Default for AllOrSome<T> {
    fn default() -> Self {
        AllOrSome::All
    }
}

impl<T> AllOrSome<T> {
    /// Returns whether this is an `All` variant.
    pub fn is_all(&self) -> bool {
        matches!(self, AllOrSome::All)
    }

    /// Returns whether this is a `Some` variant.
    #[allow(dead_code)]
    pub fn is_some(&self) -> bool {
        !self.is_all()
    }

    /// Provides a shared reference to `T` if variant is `Some`.
    pub fn as_ref(&self) -> Option<&T> {
        match *self {
            AllOrSome::All => None,
            AllOrSome::Some(ref t) => Some(t),
        }
    }

    /// Provides a mutable reference to `T` if variant is `Some`.
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match *self {
            AllOrSome::All => None,
            AllOrSome::Some(ref mut t) => Some(t),
        }
    }
}

/// Service wrapper for Cross-Origin Resource Sharing support.
///
/// This struct contains the settings for CORS requests to be validated and for responses to
/// be generated.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct CorsMiddleware<S> {
    pub(crate) service: S,
    pub(crate) inner: Rc<Inner>,
}

impl<S> CorsMiddleware<S> {
    fn handle_preflight
        (inner: &Inner, req: ServiceRequest) -> ServiceResponse<AnyBody>
    {
        if let Err(err) = inner
            .validate_origin(req.head())
            .and_then(|_| inner.validate_allowed_method(req.head()))
            .and_then(|_| inner.validate_allowed_headers(req.head()))
        {
            return req.error_response(err);
        }

        let mut res = HttpResponse::Ok();

        if let Some(origin) = inner.access_control_allow_origin(req.head()) {
            res.insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, origin));
        }

        if let Some(ref allowed_methods) = inner.allowed_methods_baked {
            res.insert_header((
                header::ACCESS_CONTROL_ALLOW_METHODS,
                allowed_methods.clone(),
            ));
        }

        if let Some(ref headers) = inner.allowed_headers_baked {
            res.insert_header((header::ACCESS_CONTROL_ALLOW_HEADERS, headers.clone()));
        } else if let Some(headers) =
            req.headers().get(header::ACCESS_CONTROL_REQUEST_HEADERS)
        {
            // all headers allowed, return
            res.insert_header((header::ACCESS_CONTROL_ALLOW_HEADERS, headers.clone()));
        }

        if inner.supports_credentials {
            res.insert_header((
                header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                HeaderValue::from_static("true"),
            ));
        }

        if let Some(max_age) = inner.max_age {
            res.insert_header((header::ACCESS_CONTROL_MAX_AGE, max_age.to_string()));
        }

        req.into_response(res.finish())
    }

    fn augment_response(
        inner: &Inner,
        mut res: ServiceResponse<AnyBody>,
    ) -> ServiceResponse<AnyBody> {
        if let Some(origin) = inner.access_control_allow_origin(res.request().head()) {
            res.headers_mut()
                .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
        };

        if let Some(ref expose) = inner.expose_headers_baked {
            res.headers_mut()
                .insert(header::ACCESS_CONTROL_EXPOSE_HEADERS, expose.clone());
        }

        if inner.supports_credentials {
            res.headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                HeaderValue::from_static("true"),
            );
        }

        if inner.vary_header {
            let value = match res.headers_mut().get(header::VARY) {
                Some(hdr) => {
                    let mut val: Vec<u8> = Vec::with_capacity(hdr.len() + 8);
                    val.extend(hdr.as_bytes());
                    val.extend(b", Origin");
                    val.try_into().unwrap()
                }
                None => HeaderValue::from_static("Origin"),
            };

            res.headers_mut().insert(header::VARY, value);
        }

        res
    }
}

type CorsMiddlewareServiceFuture = Either<
    Ready<Result<ServiceResponse<AnyBody>, actix_web::Error>>,
    LocalBoxFuture<'static, Result<ServiceResponse<AnyBody>, actix_web::Error>>,
>;
/*
impl<S> Service<ServiceRequest> for CorsMiddleware<S>
where
    S: Service<ServiceRequest,
    Response = ServiceResponse<AnyBody>,
    Error = actix_web::Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<AnyBody>;
    type Error = Error;
    type Future = CorsMiddlewareServiceFuture<AnyBody>;

    fn poll_ready(&self, ctx: &mut futures::task::Context<'_>) -> futures::task::Poll<Result<(), Self::Error>> {
        futures::task::Poll::Ready(Ok(()))
    }
    fn call(&self, req: ServiceRequest) -> Self::Future {
        if self.inner.preflight && req.method() == Method::OPTIONS {
            let inner = Rc::clone(&self.inner);
            let res = Self::handle_preflight(&inner, req);
            Either::Left(ok(res))
        } else {
            let origin = req.headers().get(header::ORIGIN).cloned();

            if origin.is_some() {
                // Only check requests with a origin header.
                if let Err(err) = self.inner.validate_origin(req.head()) {
                    debug!("origin validation failed; inner service is not called");
                    return Either::Left(ok(req.error_response(err)));
                }
            }

            let inner = Rc::clone(&self.inner);
            let fut = self.service.call(req);

            let res = async move {
                let res = fut.await;

                if origin.is_some() {
                    let res = res?;
                    Ok(Self::augment_response(&inner, res))
                } else {
                    res
                }
            }
            .boxed_local();

            Either::Right(res)
        }
    }
} */

impl<S> Service<ServiceRequest> for CorsMiddleware<S>
where
    S: Service<ServiceRequest,
    Response = ServiceResponse<AnyBody>,
    Error = actix_web::Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<AnyBody>;
    type Error = Error;
    type Future = CorsMiddlewareServiceFuture;

    fn poll_ready(&self, ctx: &mut futures::task::Context<'_>) -> futures::task::Poll<Result<(), Self::Error>> {
        futures::task::Poll::Ready(Ok(()))
    }
    fn call(&self, req: ServiceRequest) -> Self::Future {
        if self.inner.preflight && req.method() == Method::OPTIONS {
            let inner = Rc::clone(&self.inner);
            let res = Self::handle_preflight(&inner, req);
            Either::Left(ok(res))
        } else {
            let origin = req.headers().get(header::ORIGIN).cloned();

            if origin.is_some() {
                // Only check requests with a origin header.
                if let Err(err) = self.inner.validate_origin(req.head()) {
                    let mut res = HttpResponse::Ok()
                        .body(err.to_string());
                    debug!("origin validation failed; inner service is not called");
                    return Either::Left(ok(req.into_response(res)));
                }
            }

            let inner = Rc::clone(&self.inner);
            let fut = self.service.call(req);

            let res = async move {
                let res = fut.await;

                if origin.is_some() {
                    let res = res?;
                    Ok(Self::augment_response(&inner, res))
                } else {
                    res
                }
            }
            .boxed_local();

            Either::Right(res)
        }
    }
}


mod inner {

    use actix_web::{
        dev::RequestHead,
        error::Result,
        http::{
            header::{self, HeaderName, HeaderValue},
            Method,
        },
    };
    use once_cell::sync::Lazy;
    use tinyvec::TinyVec;
    use std::{collections::HashSet, convert::{TryFrom, TryInto}, fmt, iter::FromIterator, rc::Rc};
    use super::{AllOrSome, error::CorsError};

    #[derive(Clone)]
    pub struct OriginFn {
        pub boxed_fn: Rc<dyn Fn(&HeaderValue, &RequestHead) -> bool>,
    }

    impl Default for OriginFn {
        /// Dummy default for use in tiny_vec. Do not use.
        fn default() -> Self {
            let boxed_fn: Rc<dyn Fn(&_, &_) -> _> = Rc::new(|_origin, _req_head| false);
            Self { boxed_fn }
        }
    }

    impl fmt::Debug for OriginFn {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("origin_fn")
        }
    }

    /// Try to parse header value as HTTP method.
    fn header_value_try_into_method(hdr: &HeaderValue) -> Option<Method> {
        hdr.to_str()
            .ok()
            .and_then(|meth| Method::try_from(meth).ok())
    }

    #[derive(Debug, Clone)]
    pub(crate) struct Inner {
        pub(crate) allowed_origins: AllOrSome<HashSet<HeaderValue>>,
        pub(crate) allowed_origins_fns: TinyVec<[OriginFn; 4]>,

        pub(crate) allowed_methods: HashSet<Method>,
        pub(crate) allowed_methods_baked: Option<HeaderValue>,

        pub(crate) allowed_headers: AllOrSome<HashSet<HeaderName>>,
        pub(crate) allowed_headers_baked: Option<HeaderValue>,

        /// `All` will echo back `Access-Control-Request-Header` list.
        pub(crate) expose_headers: AllOrSome<HashSet<HeaderName>>,
        pub(crate) expose_headers_baked: Option<HeaderValue>,

        pub(crate) max_age: Option<usize>,
        pub(crate) preflight: bool,
        pub(crate) send_wildcard: bool,
        pub(crate) supports_credentials: bool,
        pub(crate) vary_header: bool,
    }

    static EMPTY_ORIGIN_SET: Lazy<HashSet<HeaderValue>> = Lazy::new(HashSet::new);

    impl Inner {
        pub(crate) fn validate_origin(&self, req: &RequestHead) -> Result<(), CorsError> {
            // return early if all origins are allowed or get ref to allowed origins set
            #[allow(clippy::mutable_key_type)]
            let allowed_origins = match &self.allowed_origins {
                AllOrSome::All if self.allowed_origins_fns.is_empty() => return Ok(()),
                AllOrSome::Some(allowed_origins) => allowed_origins,
                // only function origin validators are defined
                _ => &EMPTY_ORIGIN_SET,
            };

            // get origin header and try to parse as string
            match req.headers().get(header::ORIGIN) {
                // origin header exists and is a string
                Some(origin) => {
                    if allowed_origins.contains(origin)
                        || self.validate_origin_fns(origin, req)
                    {
                        Ok(())
                    } else {
                        Err(CorsError::OriginNotAllowed)
                    }
                }

                // origin header is missing
                // note: with our implementation, the origin header is required for OPTIONS request or
                // else this would be unreachable
                None => Err(CorsError::MissingOrigin),
            }
        }

        /// Accepts origin if _ANY_ functions return true. Only called when Origin exists.
        fn validate_origin_fns(&self, origin: &HeaderValue, req: &RequestHead) -> bool {
            self.allowed_origins_fns
                .iter()
                .any(|origin_fn| (origin_fn.boxed_fn)(origin, req))
        }

        /// Only called if origin exists and always after it's validated.
        pub(crate) fn access_control_allow_origin(
            &self,
            req: &RequestHead,
        ) -> Option<HeaderValue> {
            let origin = req.headers().get(header::ORIGIN);

            match self.allowed_origins {
                AllOrSome::All => {
                    if self.send_wildcard {
                        Some(HeaderValue::from_static("*"))
                    } else {
                        // see note below about why `.cloned()` is correct
                        origin.cloned()
                    }
                }

                AllOrSome::Some(_) => {
                    // since origin (if it exists) is known to be allowed if this method is called
                    // then cloning the option is all that is required to be used as an echoed back
                    // header value (or omitted if None)
                    origin.cloned()
                }
            }
        }

        /// Use in preflight checks and therefore operates on header list in
        /// `Access-Control-Request-Headers` not the actual header set.
        pub(crate) fn validate_allowed_method(
            &self,
            req: &RequestHead,
        ) -> Result<(), CorsError> {
            // extract access control header and try to parse as method
            let request_method = req
                .headers()
                .get(header::ACCESS_CONTROL_REQUEST_METHOD)
                .map(header_value_try_into_method);

            match request_method {
                // method valid and allowed
                Some(Some(method)) if self.allowed_methods.contains(&method) => Ok(()),

                // method valid but not allowed
                Some(Some(_)) => Err(CorsError::MethodNotAllowed),

                // method invalid
                Some(_) => Err(CorsError::BadRequestMethod),

                // method missing
                None => Err(CorsError::MissingRequestMethod),
            }
        }

        pub(crate) fn validate_allowed_headers(
            &self,
            req: &RequestHead,
        ) -> Result<(), CorsError> {
            // return early if all headers are allowed or get ref to allowed origins set
            #[allow(clippy::mutable_key_type)]
            let allowed_headers = match &self.allowed_headers {
                AllOrSome::All => return Ok(()),
                AllOrSome::Some(allowed_headers) => allowed_headers,
            };

            // extract access control header as string
            // header format should be comma separated header names
            let request_headers = req
                .headers()
                .get(header::ACCESS_CONTROL_REQUEST_HEADERS)
                .map(|hdr| hdr.to_str());

            match request_headers {
                // header list is valid string
                Some(Ok(headers)) => {
                    // the set is ephemeral we take care not to mutate the
                    // inserted keys so this lint exception is acceptable
                    #[allow(clippy::mutable_key_type)]
                    let mut request_headers = HashSet::with_capacity(8);

                    // try to convert each header name in the comma-separated list
                    for hdr in headers.split(',') {
                        match hdr.trim().try_into() {
                            Ok(hdr) => request_headers.insert(hdr),
                            Err(_) => return Err(CorsError::BadRequestHeaders),
                        };
                    }

                    // header list must contain 1 or more header name
                    if request_headers.is_empty() {
                        return Err(CorsError::BadRequestHeaders);
                    }

                    // request header list must be a subset of allowed headers
                    if !request_headers.is_subset(allowed_headers) {
                        return Err(CorsError::HeadersNotAllowed);
                    }

                    Ok(())
                }

                // header list is not a string
                Some(Err(_)) => Err(CorsError::BadRequestHeaders),

                // header list missing
                None => Ok(()),
            }
        }
    }

}

pub mod builder {
    use std::{collections::HashSet, convert::TryInto, iter::FromIterator, rc::Rc};

    use actix_web::{
        dev::{RequestHead, Service, ServiceRequest, ServiceResponse, Transform},
        error::{Error, Result},
        http::{self, header::HeaderName, Error as HttpError, HeaderValue, Method, Uri},
        Either,
    };
    use actix_web::body::AnyBody;
    use futures_util::future::{self, Ready};
    use log::error;
    use once_cell::sync::Lazy;
    use tinyvec::tiny_vec;

    use super::{AllOrSome, error::CorsError, CorsMiddleware, inner::{Inner, OriginFn}};

    /// Convenience for getting mut refs to inner. Cleaner than `Rc::get_mut`.
    /// Additionally, always causes first error (if any) to be reported during initialization.
    fn cors<'a>(
        inner: &'a mut Rc<Inner>,
        err: &Option<Either<http::Error, CorsError>>,
    ) -> Option<&'a mut Inner> {
        if err.is_some() {
            return None;
        }

        Rc::get_mut(inner)
    }

    static ALL_METHODS_SET: Lazy<HashSet<Method>> = Lazy::new(|| {
        HashSet::from_iter(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
            Method::CONNECT,
            Method::PATCH,
            Method::TRACE,
        ])
    });

    /// Builder for CORS middleware.
    ///
    /// To construct a CORS middleware, call [`Cors::default()`] to create a blank, restrictive builder.
    /// Then use any of the builder methods to customize CORS behavior.
    ///
    /// The alternative [`Cors::permissive()`] constructor is available for local development, allowing
    /// all origins and headers, etc. **The permissive constructor should not be used in production.**
    ///
    /// # Errors
    /// Errors surface in the middleware initialization phase. This means that, if you have logs enabled
    /// in Actix Web (using `env_logger` or other crate that exposes logs from the `log` crate), error
    /// messages will outline what is wrong with the CORS configuration in the server logs and the
    /// server will fail to start up or serve requests.
    ///
    /// # Example
    /// ```rust
    /// use actix_cors::Cors;
    /// use actix_web::http::header;
    ///
    /// let cors = Cors::default()
    ///     .allowed_origin("https://www.rust-lang.org")
    ///     .allowed_methods(vec!["GET", "POST"])
    ///     .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
    ///     .allowed_header(header::CONTENT_TYPE)
    ///     .max_age(3600);
    ///
    /// // `cors` can now be used in `App::wrap`.
    /// ```
    #[derive(Debug)]
    pub struct Cors {
        inner: Rc<Inner>,
        error: Option<Either<http::Error, CorsError>>,
    }

    impl Cors {
        /// A very permissive set of default for quick development. Not recommended for production use.
        ///
        /// *All* origins, methods, request headers and exposed headers allowed. Credentials supported.
        /// Max age 1 hour. Does not send wildcard.
        pub fn permissive() -> Self {
            let inner = Inner {
                allowed_origins: AllOrSome::All,
                allowed_origins_fns: tiny_vec![],

                allowed_methods: ALL_METHODS_SET.clone(),
                allowed_methods_baked: None,

                allowed_headers: AllOrSome::All,
                allowed_headers_baked: None,

                expose_headers: AllOrSome::All,
                expose_headers_baked: None,
                max_age: Some(3600),
                preflight: true,
                send_wildcard: false,
                supports_credentials: true,
                vary_header: true,
            };

            Cors {
                inner: Rc::new(inner),
                error: None,
            }
        }

        /// Resets allowed origin list to a state where any origin is accepted.
        ///
        /// See [`Cors::allowed_origin`] for more info on allowed origins.
        pub fn allow_any_origin(mut self) -> Cors {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                cors.allowed_origins = AllOrSome::All;
            }

            self
        }

        /// Add an origin that is allowed to make requests.
        ///
        /// By default, requests from all origins are accepted by CORS logic. This method allows to
        /// specify a finite set of origins to verify the value of the `Origin` request header.
        ///
        /// These are `origin-or-null` types in the [Fetch Standard].
        ///
        /// When this list is set, the client's `Origin` request header will be checked in a
        /// case-sensitive manner.
        ///
        /// When all origins are allowed and `send_wildcard` is set, `*` will be sent in the
        /// `Access-Control-Allow-Origin` response header. If `send_wildcard` is not set, the client's
        /// `Origin` request header will be echoed back in the `Access-Control-Allow-Origin`
        /// response header.
        ///
        /// If the origin of the request doesn't match any allowed origins and at least one
        /// `allowed_origin_fn` function is set, these functions will be used to determinate
        /// allowed origins.
        ///
        /// # Initialization Errors
        /// - If supplied origin is not valid uri
        /// - If supplied origin is a wildcard (`*`). [`Cors::send_wildcard`] should be used instead.
        ///
        /// [Fetch Standard]: https://fetch.spec.whatwg.org/#origin-header
        pub fn allowed_origin(mut self, origin: &str) -> Cors {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                match TryInto::<Uri>::try_into(origin) {
                    Ok(_) if origin == "*" => {
                        error!("Wildcard in `allowed_origin` is not allowed. Use `send_wildcard`.");
                        self.error = Some(Either::Right(CorsError::WildcardOrigin));
                    }

                    Ok(_) => {
                        if cors.allowed_origins.is_all() {
                            cors.allowed_origins =
                                AllOrSome::Some(HashSet::with_capacity(8));
                        }

                        if let Some(origins) = cors.allowed_origins.as_mut() {
                            // any uri is a valid header value
                            let hv = origin.try_into().unwrap();
                            origins.insert(hv);
                        }
                    }

                    Err(err) => {
                        self.error = Some(Either::Left(err.into()));
                    }
                }
            }

            self
        }

        /// Determinate allowed origins by processing requests which didn't match any origins specified
        /// in the `allowed_origin`.
        ///
        /// The function will receive two parameters, the Origin header value, and the `RequestHead` of
        /// each request, which can be used to determine whether to allow the request or not.
        ///
        /// If the function returns `true`, the client's `Origin` request header will be echoed back
        /// into the `Access-Control-Allow-Origin` response header.
        pub fn allowed_origin_fn<F>(mut self, f: F) -> Cors
        where
            F: (Fn(&HeaderValue, &RequestHead) -> bool) + 'static,
        {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                cors.allowed_origins_fns.push(OriginFn {
                    boxed_fn: Rc::new(f),
                });
            }

            self
        }

        /// Resets allowed methods list to all methods.
        ///
        /// See [`Cors::allowed_methods`] for more info on allowed methods.
        pub fn allow_any_method(mut self) -> Cors {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                cors.allowed_methods = ALL_METHODS_SET.clone();
            }

            self
        }

        /// Set a list of methods which allowed origins can perform.
        ///
        /// These will be sent in the `Access-Control-Allow-Methods` response header as specified in
        /// the [Fetch Standard CORS protocol].
        ///
        /// Defaults to `[GET, HEAD, POST, OPTIONS, PUT, PATCH, DELETE]`
        ///
        /// [Fetch Standard CORS protocol]: https://fetch.spec.whatwg.org/#http-cors-protocol
        pub fn allowed_methods<U, M>(mut self, methods: U) -> Cors
        where
            U: IntoIterator<Item = M>,
            M: TryInto<Method>,
            <M as TryInto<Method>>::Error: Into<HttpError>,
        {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                for m in methods {
                    match m.try_into() {
                        Ok(method) => {
                            cors.allowed_methods.insert(method);
                        }

                        Err(err) => {
                            self.error = Some(Either::Left(err.into()));
                            break;
                        }
                    }
                }
            }

            self
        }

        /// Resets allowed request header list to a state where any header is accepted.
        ///
        /// See [`Cors::allowed_headers`] for more info on allowed request headers.
        pub fn allow_any_header(mut self) -> Cors {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                cors.allowed_headers = AllOrSome::All;
            }

            self
        }

        /// Add an allowed request header.
        ///
        /// See [`Cors::allowed_headers`] for more info on allowed request headers.
        pub fn allowed_header<H>(mut self, header: H) -> Cors
        where
            H: TryInto<HeaderName>,
            <H as TryInto<HeaderName>>::Error: Into<HttpError>,
        {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                match header.try_into() {
                    Ok(method) => {
                        if cors.allowed_headers.is_all() {
                            cors.allowed_headers =
                                AllOrSome::Some(HashSet::with_capacity(8));
                        }

                        if let AllOrSome::Some(ref mut headers) = cors.allowed_headers {
                            headers.insert(method);
                        }
                    }

                    Err(err) => self.error = Some(Either::Left(err.into())),
                }
            }

            self
        }

        /// Set a list of request header field names which can be used when this resource is accessed by
        /// allowed origins.
        ///
        /// If `All` is set, whatever is requested by the client in `Access-Control-Request-Headers`
        /// will be echoed back in the `Access-Control-Allow-Headers` header as specified in
        /// the [Fetch Standard CORS protocol].
        ///
        /// Defaults to `All`.
        ///
        /// [Fetch Standard CORS protocol]: https://fetch.spec.whatwg.org/#http-cors-protocol
        pub fn allowed_headers<U, H>(mut self, headers: U) -> Cors
        where
            U: IntoIterator<Item = H>,
            H: TryInto<HeaderName>,
            <H as TryInto<HeaderName>>::Error: Into<HttpError>,
        {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                for h in headers {
                    match h.try_into() {
                        Ok(method) => {
                            if cors.allowed_headers.is_all() {
                                cors.allowed_headers =
                                    AllOrSome::Some(HashSet::with_capacity(8));
                            }

                            if let AllOrSome::Some(ref mut headers) = cors.allowed_headers {
                                headers.insert(method);
                            }
                        }
                        Err(err) => {
                            self.error = Some(Either::Left(err.into()));
                            break;
                        }
                    }
                }
            }

            self
        }

        /// Resets exposed response header list to a state where any header is accepted.
        ///
        /// See [`Cors::expose_headers`] for more info on exposed response headers.
        pub fn expose_any_header(mut self) -> Cors {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                cors.expose_headers = AllOrSome::All;
            }

            self
        }

        /// Set a list of headers which are safe to expose to the API of a CORS API specification.
        /// This corresponds to the `Access-Control-Expose-Headers` response header as specified in
        /// the [Fetch Standard CORS protocol].
        ///
        /// This defaults to an empty set.
        ///
        /// [Fetch Standard CORS protocol]: https://fetch.spec.whatwg.org/#http-cors-protocol
        pub fn expose_headers<U, H>(mut self, headers: U) -> Cors
        where
            U: IntoIterator<Item = H>,
            H: TryInto<HeaderName>,
            <H as TryInto<HeaderName>>::Error: Into<HttpError>,
        {
            for h in headers {
                match h.try_into() {
                    Ok(header) => {
                        if let Some(cors) = cors(&mut self.inner, &self.error) {
                            if cors.expose_headers.is_all() {
                                cors.expose_headers =
                                    AllOrSome::Some(HashSet::with_capacity(8));
                            }
                            if let AllOrSome::Some(ref mut headers) = cors.expose_headers {
                                headers.insert(header);
                            }
                        }
                    }
                    Err(err) => {
                        self.error = Some(Either::Left(err.into()));
                        break;
                    }
                }
            }

            self
        }

        /// Set a maximum time (in seconds) for which this CORS request maybe cached.
        /// This value is set as the `Access-Control-Max-Age` header as specified in
        /// the [Fetch Standard CORS protocol].
        ///
        /// Pass a number (of seconds) or use None to disable sending max age header.
        ///
        /// [Fetch Standard CORS protocol]: https://fetch.spec.whatwg.org/#http-cors-protocol
        pub fn max_age(mut self, max_age: impl Into<Option<usize>>) -> Cors {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                cors.max_age = max_age.into()
            }

            self
        }

        /// Set to use wildcard origins.
        ///
        /// If send wildcard is set and the `allowed_origins` parameter is `All`, a wildcard
        /// `Access-Control-Allow-Origin` response header is sent, rather than the request’s
        /// `Origin` header.
        ///
        /// This **CANNOT** be used in conjunction with `allowed_origins` set to `All` and
        /// `allow_credentials` set to `true`. Depending on the mode of usage, this will either result
        /// in an `CorsError::CredentialsWithWildcardOrigin` error during actix launch or runtime.
        ///
        /// Defaults to `false`.
        pub fn send_wildcard(mut self) -> Cors {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                cors.send_wildcard = true
            }

            self
        }

        /// Allows users to make authenticated requests
        ///
        /// If true, injects the `Access-Control-Allow-Credentials` header in responses. This allows
        /// cookies and credentials to be submitted across domains as specified in
        /// the [Fetch Standard CORS protocol].
        ///
        /// This option cannot be used in conjunction with an `allowed_origin` set to `All` and
        /// `send_wildcards` set to `true`.
        ///
        /// Defaults to `false`.
        ///
        /// A server initialization error will occur if credentials are allowed, but the Origin is set
        /// to send wildcards (`*`); this is not allowed by the CORS protocol.
        ///
        /// [Fetch Standard CORS protocol]: https://fetch.spec.whatwg.org/#http-cors-protocol
        pub fn supports_credentials(mut self) -> Cors {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                cors.supports_credentials = true
            }

            self
        }

        /// Disable `Vary` header support.
        ///
        /// When enabled the header `Vary: Origin` will be returned as per the Fetch Standard
        /// implementation guidelines.
        ///
        /// Setting this header when the `Access-Control-Allow-Origin` is dynamically generated
        /// (eg. when there is more than one allowed origin, and an Origin other than '*' is returned)
        /// informs CDNs and other caches that the CORS headers are dynamic, and cannot be cached.
        ///
        /// By default, `Vary` header support is enabled.
        pub fn disable_vary_header(mut self) -> Cors {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                cors.vary_header = false
            }

            self
        }

        /// Disable support for preflight requests.
        ///
        /// When enabled CORS middleware automatically handles `OPTIONS` requests.
        /// This is useful for application level middleware.
        ///
        /// By default *preflight* support is enabled.
        pub fn disable_preflight(mut self) -> Cors {
            if let Some(cors) = cors(&mut self.inner, &self.error) {
                cors.preflight = false
            }

            self
        }
    }

    impl Default for Cors {
        /// A restrictive (security paranoid) set of defaults.
        ///
        /// *No* allowed origins, methods, request headers or exposed headers. Credentials
        /// not supported. No max age (will use browser's default).
        fn default() -> Cors {
            let inner = Inner {
                allowed_origins: AllOrSome::Some(HashSet::with_capacity(8)),
                allowed_origins_fns: tiny_vec![],

                allowed_methods: HashSet::with_capacity(8),
                allowed_methods_baked: None,

                allowed_headers: AllOrSome::Some(HashSet::with_capacity(8)),
                allowed_headers_baked: None,

                expose_headers: AllOrSome::Some(HashSet::with_capacity(8)),
                expose_headers_baked: None,

                max_age: None,
                preflight: true,
                send_wildcard: false,
                supports_credentials: false,
                vary_header: true,
            };

            Cors {
                inner: Rc::new(inner),
                error: None,
            }
        }
    }


    impl<S> Transform<S, ServiceRequest> for Cors
    where
        S: Service<ServiceRequest, Response = ServiceResponse<AnyBody>, Error = Error>,
        S::Future: 'static,
    {
        type Response = ServiceResponse<AnyBody>;
        type Error = Error;
        type InitError = ();
        type Transform = CorsMiddleware<S>;
        type Future = Ready<Result<Self::Transform, Self::InitError>>;

        fn new_transform(&self, service: S) -> Self::Future {
            if let Some(ref err) = self.error {
                match err {
                    Either::Left(err) => error!("{}", err),
                    Either::Right(err) => error!("{}", err),
                }

                return future::err(());
            }

            let mut inner = Rc::clone(&self.inner);

            if inner.supports_credentials
                && inner.send_wildcard
                && inner.allowed_origins.is_all()
            {
                error!("Illegal combination of CORS options: credentials can not be supported when all \
                        origins are allowed and `send_wildcard` is enabled.");
                return future::err(());
            }

            // bake allowed headers value if Some and not empty
            match inner.allowed_headers.as_ref() {
                Some(header_set) if !header_set.is_empty() => {
                    let allowed_headers_str = intersperse_header_values(header_set);
                    Rc::make_mut(&mut inner).allowed_headers_baked =
                        Some(allowed_headers_str);
                }
                _ => {}
            }

            // bake allowed methods value if not empty
            if !inner.allowed_methods.is_empty() {
                let allowed_methods_str = intersperse_header_values(&inner.allowed_methods);
                Rc::make_mut(&mut inner).allowed_methods_baked = Some(allowed_methods_str);
            }

            // bake exposed headers value if Some and not empty
            match inner.expose_headers.as_ref() {
                Some(header_set) if !header_set.is_empty() => {
                    let expose_headers_str = intersperse_header_values(header_set);
                    Rc::make_mut(&mut inner).expose_headers_baked = Some(expose_headers_str);
                }
                _ => {}
            }

            future::ok(CorsMiddleware { service, inner })
        }
    }

    /// Only call when values are guaranteed to be valid header values and set is not empty.
    fn intersperse_header_values<T>(val_set: &HashSet<T>) -> HeaderValue
    where
        T: AsRef<str>,
    {
        val_set
            .iter()
            .fold(String::with_capacity(32), |mut acc, val| {
                acc.push_str(", ");
                acc.push_str(val.as_ref());
                acc
            })
            // set is not empty so string will always have leading ", " to trim
            [2..]
            .try_into()
            // all method names are valid header values
            .unwrap()
    }


}

pub mod error {

    use actix_web::{dev::AnyBody, HttpResponseBuilder, FromRequest, error::ResponseError};
    use actix_web::http::StatusCode;
    use actix_web::{HttpResponse, dev::Body};
    use derive_more::{Display, Error};

    /// Errors that can occur when processing CORS guarded requests.
    #[derive(Debug, Clone, Display, Error)]
    #[non_exhaustive]
    pub enum CorsError {
        /// Allowed origin argument must not be wildcard (`*`).
        #[display(fmt = "`allowed_origin` argument must not be wildcard (`*`).")]
        WildcardOrigin,

        /// Request header `Origin` is required but was not provided.
        #[display(fmt = "Request header `Origin` is required but was not provided.")]
        MissingOrigin,

        /// Request header `Access-Control-Request-Method` is required but is missing.
        #[display(
            fmt = "Request header `Access-Control-Request-Method` is required but is missing."
        )]
        MissingRequestMethod,

        /// Request header `Access-Control-Request-Method` has an invalid value.
        #[display(
            fmt = "Request header `Access-Control-Request-Method` has an invalid value."
        )]
        BadRequestMethod,

        /// Request header `Access-Control-Request-Headers` has an invalid value.
        #[display(
            fmt = "Request header `Access-Control-Request-Headers` has an invalid value."
        )]
        BadRequestHeaders,

        /// Origin is not allowed to make this request.
        #[display(fmt = "Origin is not allowed to make this request.")]
        OriginNotAllowed,

        /// Request method is not allowed.
        #[display(fmt = "Requested method is not allowed.")]
        MethodNotAllowed,

        /// One or more request headers are not allowed.
        #[display(fmt = "One or more request headers are not allowed.")]
        HeadersNotAllowed,
    }

    impl ResponseError for CorsError {
        fn status_code(&self) -> StatusCode {
            StatusCode::BAD_REQUEST
        }

        fn error_response(&self) -> HttpResponse {
            HttpResponse::BadRequest()
                .body(self.to_string())
        }

        /* fn error_response(&self) -> BaseHttpResponse<Body> {
            BaseHttpResponse::new(StatusCode::BAD_REQUEST)
                .set_body(Body::from_message(self.to_string()))
        } */

    }

}
