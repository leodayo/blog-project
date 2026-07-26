use std::sync::Arc;

use actix_web::{App, HttpServer, web};

use crate::{
    application::{auth_service::AuthService, blog_service::BlogService},
    data::{post_repository::PostRepository, user_repository::UserRepository},
    infrastructure::{
        database::{create_pool, run_migrations},
        jwt::JwtService,
    },
    presentation::{
        http_handlers::{
            create_post, delete_post, get_post, list_posts, login, register, update_post,
        },
        middleware::create_auth_middleware,
    },
};

mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let pool = create_pool(&database_url).await?;
    tracing::info!("Database connected");
    tracing::info!("Running migrations");
    run_migrations(&pool).await?;
    tracing::info!("Migrations applied successfully");

    let user_repository = Arc::new(UserRepository::new(pool.clone()));
    let post_repository = Arc::new(PostRepository::new(pool.clone()));

    let jwt_service = Arc::new(JwtService::new(&jwt_secret));
    let auth_service = Arc::new(AuthService::new(
        user_repository.clone(),
        jwt_service.clone(),
    ));
    let blog_service = Arc::new(BlogService::new(post_repository.clone()));

    HttpServer::new(move || {
        let auth_middleware = create_auth_middleware(jwt_service.clone());

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(jwt_service.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(blog_service.clone()))
            .service(
                web::scope("/api")
                    // Public routes
                    .route("/auth/register", web::post().to(register))
                    .route("/auth/login", web::post().to(login))
                    .route("/posts", web::get().to(list_posts))
                    .route("/posts/{id}", web::get().to(get_post))
                    // Protected routes
                    .service(
                        web::scope("/posts")
                            .wrap(auth_middleware)
                            .route("", web::post().to(create_post))
                            .route("/{id}", web::put().to(update_post))
                            .route("/{id}", web::delete().to(delete_post)),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
