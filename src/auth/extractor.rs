use actix_identity::RequestIdentity;
use actix_web::{dev, FromRequest, HttpRequest, Result};
use futures::future::{err, ok, Ready};

use crate::auth::decode_jwt;
use crate::error::ApiError;

#[derive(Debug)]
pub struct AuthenticationDetails {
    account_id: i32,
}

impl FromRequest for AuthenticationDetails {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        if let Some(auth_token) = req.get_identity() {
            match decode_jwt(&auth_token) {
                Ok(claim) => {
                    return ok(AuthenticationDetails {
                        account_id: claim.account_id,
                    })
                }
                Err(_) => return err(ApiError::Unauthorized),
            };
        }
        err(ApiError::Unauthorized)
    }
}
