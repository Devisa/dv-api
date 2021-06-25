// use crate::auth::jwt::Claims;
// use actix_web::{dev, Error, FromRequest, HttpRequest};
// use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

// use std::pin::Pin;
// use std::task::{Context, Poll};

// use actix_web::dev::{Service, Transform};
// use actix_web::{dev::ServiceRequest, dev::ServiceResponse, };
// use futures::future::{ok, Ready};
// use futures::Future;

// // There are two steps in middleware processing.
// // 1. Middleware initialization, middleware factory gets called with
// //    next service in chain as parameter.
// // 2. Middleware's call method gets called with normal request.
// pub struct SayHi;

// // Middleware factory is `Transform` trait from actix-service crate
// // `S` - type of the next service
// // `B` - type of response's body
// impl<S, B> Transform<> for SayHi
// where
//     S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Request = ServiceRequest;
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type InitError = ();
//     type Transform = SayHiMiddleware<S>;
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         ok(SayHiMiddleware { service })
//     }
// }

// pub struct SayHiMiddleware<S> {
//     service: S,
// }

// impl<S, B> Service for SayHiMiddleware<S>
// where
//     S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Request = ServiceRequest;
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }

//     fn call(&mut self, req: ServiceRequest) -> Self::Future {
//         println!("Hi from start. You requested: {}", req.path());

//         let fut = self.service.call(req);

//         Box::pin(async move {
//             let res = fut.await?;

//             println!("Hi from response");
//             Ok(res)
//         })
//     }
// }

// pub struct AuthorizationService;

// // impl FromRequest for AuthorizationService {
// //     type Error = Error;
// //     type Future = Ready<Result<AuthorizationService, Error>>;
// //     type Config = ();

// //     fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
// //         let _auth = _req.headers().get("Authorization");
// //         match _auth {
// //             Some(_) => {
// //                 let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
// //                 let token = _split[1].trim();
// //                 let _config: Config = Config {};
// //                 let _var = _config.get_config_with_key("SECRET_KEY");
// //                 let key = _var.as_bytes();
// //                 match decode::<Claims>(
// //                     token,
// //                     &DecodingKey::from_secret(key),
// //                     &Validation::new(Algorithm::HS256),
// //                 ) {
// //                     Ok(_token) => ok(AuthorizationService),
// //                     Err(_e) => err(ErrorUnauthorized("invalid token!")),
// //                 }
// //             }
// //             None => err(ErrorUnauthorized("blocked!")),
// //         }
// //     }
// // }
