use std::sync::Arc;

use tonic::{Request, Status};

use crate::{
    domain::user::{AuthenticatedUser, UserContext},
    infrastructure::jwt::JwtService,
};

pub fn grpc_auth_interceptor<T>(
    jwt_service: Arc<JwtService>,
) -> impl Fn(Request<T>) -> Result<Request<T>, Status> + Clone {
    move |mut req: Request<T>| {
        let user = req
            .metadata()
            .get("authorization")
            .and_then(|auth_header| auth_header.to_str().ok())
            .and_then(|auth_str| auth_str.strip_prefix("Bearer "))
            .and_then(|token| jwt_service.verify_token(token).ok())
            .map(|claims| AuthenticatedUser {
                id: claims.user_id,
                username: claims.username,
            });

        if let Some(authenticated_user) = user {
            req.extensions_mut().insert(authenticated_user);
        }

        Ok(req)
    }
}

pub trait TonicAuthExt {
    // UserContext is not used as of now,
    // yet it's something extremely common
    // that will most likely be widely used
    // in futute if this project will be
    // maintained
    #[allow(dead_code)]
    fn user_context(&self) -> UserContext;
    fn require_authenticated(&self) -> Result<UserContext, Status>;
}

impl<T> TonicAuthExt for Request<T> {
    fn user_context(&self) -> UserContext {
        self.extensions()
            .get::<AuthenticatedUser>()
            .cloned()
            .map(UserContext::Authenticated)
            .unwrap_or(UserContext::Anonymous)
    }

    fn require_authenticated(&self) -> Result<UserContext, Status> {
        self.extensions()
            .get::<AuthenticatedUser>()
            .cloned()
            .map(UserContext::Authenticated)
            .ok_or_else(|| Status::unauthenticated("Authentication required"))
    }
}
