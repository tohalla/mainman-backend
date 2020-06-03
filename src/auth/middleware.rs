use actix_identity::{CookieIdentityPolicy, IdentityService};

pub fn default() -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(
            std::env::var("SESSION_KEY").unwrap().as_ref(),
        )
        .name("authorization")
        .secure(false),
    )
}
