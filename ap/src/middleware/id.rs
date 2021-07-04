////! For middleware documentation, see [`Logger`].

//use std::{
//    collections::HashSet,
//    convert::TryFrom,
//    env,
//    fmt::{self, Display as _},
//    future::Future,
//    marker::PhantomData,
//    pin::Pin,
//    rc::Rc,
//    task::{Context, Poll},
//};

//use actix_service::{Service, Transform};
//use actix::fut::{ok, Ready};
//use futures::ready;
//use bytes::Bytes;
//use log::{debug, warn};
//use regex::{Regex, RegexSet};
//use time::OffsetDateTime;

//use actix_web::{
//    dev::{ServiceRequest, ServiceResponse, BodySize, MessageBody, ResponseBody},
//    error::{Error, Result},
//    http::{HeaderName, StatusCode},
//    HttpResponse,
//};

//#[derive(Debug)]
//pub struct Logger(Rc<Inner>);

//#[derive(Debug, Clone)]
//struct Inner {
//    format: Format,
//    exclude: HashSet<String>,
//    exclude_regex: RegexSet,
//}

//impl Logger {
//    /// Create `Logger` middleware with the specified `format`.
//    pub fn new(format: &str) -> Logger {
//        Logger(Rc::new(Inner {
//            format: Format::new(format),
//            exclude: HashSet::new(),
//            exclude_regex: RegexSet::empty(),
//        }))
//    }

//    /// Ignore and do not log access info for specified path.
//    pub fn exclude<T: Into<String>>(mut self, path: T) -> Self {
//        Rc::get_mut(&mut self.0)
//            .unwrap()
//            .exclude
//            .insert(path.into());
//        self
//    }

//    /// Ignore and do not log access info for paths that match regex.
//    pub fn exclude_regex<T: Into<String>>(mut self, path: T) -> Self {
//        let inner = Rc::get_mut(&mut self.0).unwrap();
//        let mut patterns = inner.exclude_regex.patterns().to_vec();
//        patterns.push(path.into());
//        let regex_set = RegexSet::new(patterns).unwrap();
//        inner.exclude_regex = regex_set;
//        self
//    }

//    /// Register a function that receives a ServiceRequest and returns a String for use in the
//    /// log line. The label passed as the first argument should match a replacement substring in
//    /// the logger format like `%{label}xi`.
//    ///
//    /// It is convention to print "-" to indicate no output instead of an empty string.
//    ///
//    /// # Example
//    /// ```
//    /// # use actix_web::{http::HeaderValue, middleware::Logger};
//    /// # fn parse_jwt_id (_req: Option<&HeaderValue>) -> String { "jwt_uid".to_owned() }
//    /// Logger::new("example %{JWT_ID}xi")
//    ///     .custom_request_replace("JWT_ID", |req| parse_jwt_id(req.headers().get("Authorization")));
//    /// ```
//    pub fn custom_request_replace(
//        mut self,
//        label: &str,
//        f: impl Fn(&ServiceRequest) -> String + 'static,
//    ) -> Self {
//        let inner = Rc::get_mut(&mut self.0).unwrap();

//        let ft = inner.format.0.iter_mut().find(
//            |ft| matches!(ft, FormatText::CustomRequest(unit_label, _) if label == unit_label),
//        );

//        if let Some(FormatText::CustomRequest(_, request_fn)) = ft {
//            // replace into None or previously registered fn using same label
//            request_fn.replace(CustomRequestFn {
//                inner_fn: Rc::new(f),
//            });
//        } else {
//            // non-printed request replacement function diagnostic
//            debug!(
//                "Attempted to register custom request logging function for nonexistent label: {}",
//                label
//            );
//        }

//        self
//    }
//}

//impl<S, B> Transform<S, ServiceRequest> for Logger
//where
//    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//    B: MessageBody,
//{
//    type Response = ServiceResponse<StreamLog<B>>;
//    type Error = Error;
//    type InitError = ();
//    type Transform = LoggerMiddleware<S>;
//    type Future = Ready<Result<Self::Transform, Self::InitError>>;

//    fn new_transform(&self, service: S) -> Self::Future {
//        for unit in &self.0.format.0 {
//            // missing request replacement function diagnostic
//            if let FormatText::CustomRequest(label, None) = unit {
//                warn!(
//                    "No custom request replacement function was registered for label \"{}\".",
//                    label
//                );
//            }
//        }

//        ok(LoggerMiddleware {
//            service,
//            inner: self.0.clone(),
//        })
//    }
//}

///// Logger middleware service.
//pub struct LoggerMiddleware<S> {
//    inner: Rc<Inner>,
//    service: S,
//}

//impl<S, B> Service<ServiceRequest> for LoggerMiddleware<S>
//where
//    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//    B: MessageBody,
//{
//    type Response = ServiceResponse<StreamLog<B>>;
//    type Error = Error;
//    type Future = LoggerResponse<S, B>;

//    actix_service::forward_ready!(service);

//    fn call(&self, req: ServiceRequest) -> Self::Future {
//        if self.inner.exclude.contains(req.path())
//            || self.inner.exclude_regex.is_match(req.path())
//        {
//            LoggerResponse {
//                fut: self.service.call(req),
//                format: None,
//                time: OffsetDateTime::now_utc(),
//                _phantom: PhantomData,
//            }
//        } else {
//            let now = OffsetDateTime::now_utc();
//            let mut format = self.inner.format.clone();

//            for unit in &mut format.0 {
//                unit.render_request(now, &req);
//            }
//            LoggerResponse {
//                fut: self.service.call(req),
//                format: Some(format),
//                time: now,
//                _phantom: PhantomData,
//            }
//        }
//    }
//}

