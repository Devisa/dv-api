use actix_web::{
    guard::Guard,
    dev::{Payload,RequestHead,PayloadStream},
    Error, FromRequest, HttpRequest,
};

use std::{
    future::Future,
    pin::Pin
};

#[derive(Clone)]
pub struct AuthInfo {
    pub permissions: Vec<String>
}

impl AuthInfo {
    pub fn new(permissions: Vec<String>) -> AuthInfo {
        AuthInfo { permissions }
    }
}

pub trait PermissionsCheck {
    fn has_permission(&self, permission: &str) -> bool;
    fn has_permissions(&self, permissions: Vec<&str>) -> bool;
    fn has_any_permission(&self, permissions: Vec<&str>) -> bool;
}

impl PermissionsCheck for AuthInfo {
    fn has_permission(&self, permission: &str) -> bool {
        self.permissions
            .iter()
            .any(|auth| auth.as_str() == permission)
    }

    fn has_permissions(&self, permissions: Vec<&str>) -> bool {
        permissions
            .into_iter()
            .all(|auth| self.has_permission(auth))
    }

    fn has_any_permission(&self, permissions: Vec<&str>) -> bool {
        permissions
            .into_iter()
            .any(|auth| self.has_permission(auth))
    }
}

pub struct PermissionGuard {
    allow_permission: String,
}

impl PermissionGuard {
    pub fn new(allow_permission: String) -> PermissionGuard {
        PermissionGuard { allow_permission }
    }
}

impl Guard for PermissionGuard {
    fn check(&self, request: &RequestHead) -> bool {
        request
            .extensions()
            .get::<AuthInfo>()
            .filter(|Info| Info.has_permission(self.allow_permission.as_str()))
            .is_some()
    }
}
impl FromRequest for AuthInfo {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload<PayloadStream>) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            req.extensions()
                .get::<Self>()
                .map(Self::clone)
                .unwrap()
        })
    }
}
