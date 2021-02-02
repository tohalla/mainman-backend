use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web::Data,
    HttpMessage, HttpResponse,
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

use super::*;
use crate::{
    db::{Connection, Pool},
    error::Error,
    schema::{organisation, organisation_account},
    MainmanResult,
};

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
        Error = actix_web::Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
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

fn check_access(
    claim: &Claim,
    path_info: &PathInfo,
    conn: &Connection,
) -> MainmanResult<()> {
    check_account(&claim, &path_info)
        .and_then(|_| check_organisation(&claim, &path_info, conn))
}

fn check_organisation(
    claim: &Claim,
    path_info: &PathInfo,
    conn: &Connection,
) -> MainmanResult<()> {
    match path_info.organisation_id {
        Some(organisation_id) => {
            let admin_account = organisation::dsl::organisation
                .left_join(
                    organisation_account::table
                        .on(organisation_account::account.eq(claim.account_id)),
                )
                .filter(organisation::dsl::id.eq(organisation_id))
                .select(organisation::admin_account)
                .first::<i32>(conn);
            if let Ok(admin_account) = admin_account {
                if admin_account == claim.account_id {
                    return Ok(());
                }
            }
            Err(Error::UnauthorizedError)
        }
        None => Ok(()),
    }
}

fn check_account(claim: &Claim, path_info: &PathInfo) -> MainmanResult<()> {
    if let Some(account_id) = path_info.account_id {
        if claim.account_id != account_id {
            return Err(Error::UnauthorizedError);
        }
    }
    Ok(())
}

impl<'a, S, B> Service for RequireAuthenticationMiddleware<'a, S>
where
    S: Service<
            Request = ServiceRequest,
            Response = ServiceResponse<B>,
            Error = actix_web::Error,
        > + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
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
        let conn = &req.app_data::<Data<Pool>>().unwrap().get().unwrap();

        let mut service = self.service.clone();
        let authentication_token = req
            .cookie("authorization")
            .map(|cookie| cookie.value().to_string());

        if let Ok(claim) = Claim::from_identity(authentication_token.to_owned())
        {
            if check_access(&claim, &path_info, conn).is_ok() {
                return Box::pin(async move { Ok(service.call(req).await?) });
            }
        }
        if let Some(refresh_token) = req.cookie("refresh-token") {
            if let Ok(refresh_token) = Uuid::parse_str(refresh_token.value()) {
                if let Ok(claim) = super::RefreshToken(refresh_token)
                    .validate_refresh_token(
                        authentication_token.and_then(|auth_token| {
                            AuthCookies::parse_auth_token(&auth_token)
                        }),
                        conn,
                    )
                {
                    if check_access(&claim, &path_info, conn).is_ok() {
                        if let Ok(cookies) = AuthCookies::cookies(&claim, conn)
                        {
                            return Box::pin(async move {
                                let mut res = service.call(req).await?;
                                res.response_mut().add_cookie(&cookies.auth)?;
                                res.response_mut()
                                    .add_cookie(&cookies.refresh)?;
                                Ok(res)
                            });
                        }
                    }
                }
            }
        }
        Box::pin(ok(req.into_response(
            HttpResponse::Unauthorized().finish().into_body(),
        )))
    }
}
