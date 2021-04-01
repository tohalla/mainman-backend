use actix_http::http::StatusCode;
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
    organisation::OrganisationAccount,
    schema::organisation_account,
    MainmanResult,
};

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub account_id: Option<i64>,
    pub organisation_id: Option<i64>,
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
    check_account(&claim, &path_info)?;
    if let Some(organisation_id) = path_info.organisation_id {
        check_organisation_access(claim, organisation_id, conn)?;
    }
    Ok(())
}

fn check_organisation_access(
    claim: &Claim,
    organisation_id: i64,
    conn: &Connection,
) -> MainmanResult<()> {
    organisation_account::table
        .filter(
            organisation_account::organisation
                .eq(organisation_id)
                .and(organisation_account::account.eq(claim.account_id)),
        )
        .first::<OrganisationAccount>(conn)?;

    Ok(())
}

fn check_account(claim: &Claim, path_info: &PathInfo) -> MainmanResult<()> {
    if let Some(account_id) = path_info.account_id {
        if claim.account_id != account_id {
            return Err(StatusCode::FORBIDDEN.into());
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

        if let Ok(claim) = Claim::from_identity(
            authentication_token.to_owned(),
            &Validation::default(),
        ) {
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
