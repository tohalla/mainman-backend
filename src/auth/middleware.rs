use actix_identity::{CookieIdentityPolicy, IdentityService, RequestIdentity};
use actix_service::{Service, Transform};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse,
};
use diesel::prelude::*;
use futures::{
    future::{ok, Ready},
    Future,
};
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use super::AuthenticationDetails;
use crate::db::Pool;
use crate::schema::{organisation, organisation_account};

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

pub struct RequireAuthentication<'a> {
    require_permissions: Option<&'a [&'a str]>,
}

impl<'a> Default for RequireAuthentication<'a> {
    fn default() -> RequireAuthentication<'a> {
        RequireAuthentication {
            require_permissions: None,
        }
    }
}

impl<'a, S: 'static, B> Transform<S> for RequireAuthentication<'a>
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
    type Transform = RequireAuthenticationMiddleware<'a, S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequireAuthenticationMiddleware {
            service: Rc::new(RefCell::new(service)),
            require_permissions: self.require_permissions,
        })
    }
}

#[allow(dead_code)]
pub struct RequireAuthenticationMiddleware<'a, S> {
    service: Rc<RefCell<S>>,
    require_permissions: Option<&'a [&'a str]>,
}

fn check_organisation(
    pool: &Pool,
    authentication_details: &AuthenticationDetails,
    path_info: &PathInfo,
) -> Result<bool, Error> {
    match path_info.organisation_id {
        Some(organisation_id) => {
            let conn = pool.get().unwrap();
            let admin_account = organisation::dsl::organisation
                .left_join(
                    organisation_account::table.on(
                        organisation_account::organisation
                            .eq(organisation_id)
                            .and(
                                organisation_account::account
                                    .eq(authentication_details.account_id),
                            ),
                    ),
                )
                .select(organisation::admin_account)
                .first::<i32>(&conn);
            if let Ok(admin_account) = admin_account {
                return Ok(admin_account == authentication_details.account_id);
            }
            Ok(false)
        }
        None => Ok(true),
    }
}

fn check_account(
    authentication_details: &AuthenticationDetails,
    path_info: &PathInfo,
) -> bool {
    if let Some(account_id) = path_info.account_id {
        return authentication_details.account_id == account_id;
    }
    true
}

impl<'a, S, B> Service for RequireAuthenticationMiddleware<'a, S>
where
    S: Service<
            Request = ServiceRequest,
            Response = ServiceResponse<B>,
            Error = Error,
        > + 'static,
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
        ctx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let path_info = req
            .match_info()
            .load::<PathInfo>()
            .unwrap_or(PathInfo::default());
        let pool = req.app_data::<Pool>().unwrap();

        let mut service = self.service.clone();
        if let Ok(authentication_details) = AuthenticationDetails::from_identity(
            RequestIdentity::get_identity(&req),
        ) {
            if check_account(&authentication_details, &path_info)
                && check_organisation(
                    &pool,
                    &authentication_details,
                    &path_info,
                )
                .unwrap_or(false)
            {
                return Box::pin(async move { Ok(service.call(req).await?) });
            }
        }

        Box::pin(ok(req.into_response(
            HttpResponse::Unauthorized().finish().into_body(),
        )))
    }
}
