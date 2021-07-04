use futures_util::future::{self, Ready};
use std::{convert::TryInto,  rc::Rc};
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
    use std::{collections::HashSet, convert::TryFrom, fmt, iter::FromIterator, };
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    error::Error,
    HttpResponse,
};
use futures::poll;
use futures_util::future::{ok, Either, FutureExt as _, LocalBoxFuture, };
use log::debug;

#[derive(Debug, Clone)]
pub struct CorsMiddleware<S> {
    pub(crate) service: S,
    pub(crate) inner: Rc<Inner>,
}

impl<S> CorsMiddleware<S> {

}

type CorsMiddlewareServiceFuture<B> = Either<
    Ready<Result<ServiceResponse<B>, Error>>,
    Ready<Result<ServiceResponse<B>, Error>>,
>;

impl<S, B> Service<ServiceRequest> for CorsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = CorsMiddlewareServiceFuture<B>;

    fn poll_ready(&self, ctx: &mut futures::task::Context<'_>) -> futures::task::Poll<Result<(), Self::Error>> {
        futures::task::Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future
    {
        let mut res = HttpResponse::Ok();
        if let Some(he) = req.headers().get("dvsa-auth")
        {
            log::info!("USER HAS dvsa-auth HEADERS");
            if let Ok(claims) = crate::auth::jwt::decode_token_alt(he.to_str().expect("Could not parse to str"))
            {
                if claims.exp > 0
                {
                    log::info!("USER IS AUTHORIZED AT ROLE {}", claims.role);
                    res.insert_header(("dvsa-authorized", "false"));
                    let resp = res.finish();
                    let resp = resp.into_body();
                    let r = req.into_response(resp);
                    return Either::Right(ok(r));
                }
                else
                {
                    log::info!("USER HAS EXPIRED JWT.");
                    res.insert_header(("dvsa-authorized", "true"));
                    let mut resp = HttpResponse::Unauthorized();
                    let resp = resp.finish();
                    let resp = resp.into_body();
                    let r = req.into_response(resp);
                    return Either::Left(ok(r));
                }
            }
            else
            {
                log::info!("USER IS UNAUTHORIZED - NO dvsa-auth HEADER");
                res.insert_header(("dvsa-authorized", "true"));
                let mut resp = HttpResponse::Unauthorized();
                let resp = resp.finish();
                let resp = resp.into_body();
                let r = req.into_response(resp);
                return Either::Left(ok(r));
            }
        }
        else
        {
            log::info!("USER IS UNAUTHORIZED - NO dvsa-auth HEADER");
            res.insert_header(("dvsa-authorized", "true"));

            let mut resp = HttpResponse::Unauthorized();
            let resp = resp.finish();
            let resp = resp.into_body();
            let r = req.into_response(resp);
            return Either::Left(ok(r));
        }
    }
}

#[derive(Debug)]
pub struct Cors {
    inner: Rc<Inner>,
}

#[derive(Debug)]
pub struct Inner {

}

impl<S, B> actix_web::dev::Transform<S, ServiceRequest> for Cors
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CorsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let mut inner = Rc::clone(&self.inner);
        future::ok( CorsMiddleware { service, inner }  )
    }
}
