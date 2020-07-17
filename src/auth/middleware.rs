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

pub struct RequireAuthentication;

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
        ok(RequireAuthenticationMiddleware { service })
    }
}

pub struct RequireAuthenticationMiddleware<S> {
    service: S,
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
        if let Ok(_) =
            super::AuthenticationDetails::from_identity(req.get_identity())
        {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        };

        Box::pin(async move {
            Ok(req.into_response(
                HttpResponse::Unauthorized().finish().into_body(),
            ))
        })
    }
}
