use std::sync::Arc;

use actix_web::{HttpMessage, dev::ServiceRequest};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};

use crate::infrastructure::jwt::JwtService;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i64,
    pub username: String,
}

pub fn create_auth_middleware(
    jwt_service: Arc<JwtService>,
) -> HttpAuthentication<
    BearerAuth,
    impl Fn(
        ServiceRequest,
        BearerAuth,
    ) -> std::future::Ready<Result<ServiceRequest, (actix_web::Error, ServiceRequest)>>,
> {
    HttpAuthentication::bearer(move |req: ServiceRequest, auth: BearerAuth| {
        let Ok(claims) = jwt_service.verify_token(auth.token()) else {
            let err = actix_web::error::ErrorUnauthorized("Invalid or expired token");
            return std::future::ready(Err((err, req)));
        };

        let user = AuthenticatedUser {
            user_id: claims.user_id,
            username: claims.username,
        };
        req.extensions_mut().insert(user);

        std::future::ready(Ok(req))
    })
}
