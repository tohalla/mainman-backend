use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService, RequestIdentity};
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::pin::Pin;
use std::task::{Context, Poll};

use super::AuthenticationDetails;

pub fn default() -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(
            std::env::var("SESSION_KEY").unwrap().as_ref(),
        )
        .name("authorization")
        .same_site(SameSite::Strict)
        .secure(false),
    )
}

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub account_id: Option<i32>,
    pub organisation_id: Option<i32>,
}

impl Default for PathInfo {
    fn default() -> PathInfo {
        PathInfo {
            account_id: None,
            organisation_id: None,
        }
    }
}

pub struct RequireAuthentication {
    pub validate: fn(PathInfo, AuthenticationDetails) -> bool,
}

impl Default for RequireAuthentication {
    fn default() -> RequireAuthentication {
        RequireAuthentication {
            validate: |_, _| true,
        }
    }
}

impl<S, B> Transform<S> for RequireAuthentication
where
    S: Service<
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequireAuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequireAuthenticationMiddleware {
            service,
            validate: self.validate,
        })
    }
}

pub struct RequireAuthenticationMiddleware<S> {
    service: S,
    validate: fn(PathInfo, AuthenticationDetails) -> bool,
}

impl<S, B> Service for RequireAuthenticationMiddleware<S>
where
    S: Service<
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if let Ok(authentication_details) =
            AuthenticationDetails::from_identity(req.get_identity())
        {
            if (self.validate)(
                req.match_info()
                    .load::<PathInfo>()
                    .unwrap_or(PathInfo::default()),
                authentication_details,
            ) {
                let fut = self.service.call(req);
                return Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                });
            }
        };

        Box::pin(async move {
            Ok(req.into_response(
                HttpResponse::Unauthorized().finish().into_body(),
            ))
        })
    }
}
