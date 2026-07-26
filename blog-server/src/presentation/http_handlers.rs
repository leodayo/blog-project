use actix_web::{HttpResponse, web};
use std::sync::Arc;

use crate::{
    application::{
        auth_service::AuthService,
        blog_service::BlogService,
        dto::{
            auth::{AuthResponse, LoginRequest, RegisterUserRequest},
            common::Pagination,
            post::{CreatePostRequest, ListPostsResponse, UpdatePostRequest},
        },
    },
    domain::error::AppError,
    presentation::middleware::UserContext,
};

pub async fn register(
    auth_service: web::Data<Arc<AuthService>>,
    form: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse, AppError> {
    let (token, user) = auth_service
        .register(&form.username, &form.email, &form.password)
        .await?;

    Ok(HttpResponse::Created().json(AuthResponse { token, user }))
}

pub async fn login(
    auth_service: web::Data<Arc<AuthService>>,
    form: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let (token, user) = auth_service.login(&form.username, &form.password).await?;

    Ok(HttpResponse::Ok().json(AuthResponse { token, user }))
}

pub async fn get_post(
    post_id: web::Path<i64>,
    blog_service: web::Data<Arc<BlogService>>,
) -> Result<HttpResponse, AppError> {
    let post_id = post_id.into_inner();
    let post = blog_service.get_post(post_id).await?;

    Ok(HttpResponse::Ok().json(post))
}

pub async fn create_post(
    user_context: UserContext,
    form: web::Json<CreatePostRequest>,
    blog_service: web::Data<Arc<BlogService>>,
) -> Result<HttpResponse, AppError> {
    let post = blog_service
        .create_post(user_context, &form.title, &form.content)
        .await?;

    Ok(HttpResponse::Created().json(post))
}

pub async fn update_post(
    post_id: web::Path<i64>,
    form: web::Json<UpdatePostRequest>,
    user_context: UserContext,
    blog_service: web::Data<Arc<BlogService>>,
) -> Result<HttpResponse, AppError> {
    let post_id = post_id.into_inner();
    let post = blog_service
        .update_post(post_id, user_context, &form.title, &form.content)
        .await?;

    Ok(HttpResponse::Ok().json(post))
}

pub async fn delete_post(
    post_id: web::Path<i64>,
    user_context: UserContext,
    blog_service: web::Data<Arc<BlogService>>,
) -> Result<HttpResponse, AppError> {
    let post_id = post_id.into_inner();
    blog_service.delete_post(post_id, user_context).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn list_posts(
    query: web::Query<Pagination>,
    blog_service: web::Data<Arc<BlogService>>,
) -> Result<HttpResponse, AppError> {
    let (posts, total) = blog_service.get_posts(query.limit, query.offset).await?;

    Ok(HttpResponse::Ok().json(ListPostsResponse {
        posts,
        total,
        limit: query.limit,
        offset: query.offset,
    }))
}
