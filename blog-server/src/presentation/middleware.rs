use std::{future, sync::Arc};

use actix_web::{FromRequest, HttpMessage, dev::ServiceRequest};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};

use crate::infrastructure::jwt::JwtService;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: i64,
    pub username: String,
}

#[derive(Debug, Clone)]
pub enum UserContext {
    Authenticated(AuthenticatedUser),
    Anonymous,
}

impl FromRequest for UserContext {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        if let Some(user) = req.extensions().get::<AuthenticatedUser>() {
            let authenticated = UserContext::Authenticated(user.clone());
            future::ready(Ok(authenticated))
        } else {
            // TODO() that also means that if someone tries extracting
            // UserContext in a public handler they will get ::Anonymous
            // even if the user is authentificated, which is probably fine
            // for now, but might be not that obvious. Maybe it's better to
            // separate auth_middleware into two.. and apply one on the
            // whole service, which would put UserContext in the context,
            // and a separate one that would protect private endpoints,
            // that separate middleware would check the context and deny
            // ::Anonymous users.
            // I feel like it's an overkill for now, so I'll leave it as is
            // and later consider what's the best course of action here.
            // Suggestions/advises are much appreaciated.
            future::ready(Ok(UserContext::Anonymous))
        }
    }
}

pub fn create_auth_middleware(
    jwt_service: Arc<JwtService>,
) -> HttpAuthentication<
    BearerAuth,
    impl Fn(
        ServiceRequest,
        BearerAuth,
    ) -> future::Ready<Result<ServiceRequest, (actix_web::Error, ServiceRequest)>>,
> {
    HttpAuthentication::bearer(move |req: ServiceRequest, auth: BearerAuth| {
        let Ok(claims) = jwt_service.verify_token(auth.token()) else {
            let err = actix_web::error::ErrorUnauthorized("Invalid or expired token");
            return future::ready(Err((err, req)));
        };

        let user = AuthenticatedUser {
            id: claims.user_id,
            username: claims.username,
        };
        req.extensions_mut().insert(user);

        future::ready(Ok(req))
    })
}
