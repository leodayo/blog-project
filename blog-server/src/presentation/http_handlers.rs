use std::sync::Arc;

use actix_web::{HttpResponse, web};

use crate::{
    application::{
        auth_service::AuthService,
        dto::auth::{AuthResponse, LoginRequest, RegisterUserRequest},
    },
    domain::error::AppError,
};

pub async fn register(
    auth_service: web::Data<Arc<AuthService>>,
    form: web::Json<RegisterUserRequest>,
) -> HttpResponse {
    match auth_service
        .register(&form.username, &form.email, &form.password)
        .await
    {
        Ok((token, user)) => HttpResponse::Created().json(AuthResponse { token, user }),
        Err(AppError::UserAlreadyExists) => HttpResponse::Conflict().body("User already exists"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn login(
    auth_service: web::Data<Arc<AuthService>>,
    form: web::Json<LoginRequest>,
) -> HttpResponse {
    match auth_service.login(&form.username, &form.password).await {
        Ok((token, user)) => HttpResponse::Ok().json(AuthResponse { token, user }),
        Err(AppError::InvalidCredentials) => {
            HttpResponse::Unauthorized().body("Invalid credentials")
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_post() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

pub async fn get_post() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

pub async fn update_post() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

pub async fn delete_post() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

pub async fn list_posts() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}
