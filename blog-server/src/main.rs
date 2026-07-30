use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, web};
use blog_proto::blog_service_server::BlogServiceServer;

use crate::{
    application::{auth_service::AuthService, blog_service::BlogService},
    data::{post_repository::PostRepository, user_repository::UserRepository},
    infrastructure::{
        database::{create_pool, run_migrations},
        jwt::JwtService,
        logging::configure_logger,
    },
    presentation::{
        actix_middleware::create_auth_middleware,
        grpc_middleware::grpc_auth_interceptor,
        grpc_service::BlogGrpcService,
        http_handlers::{
            create_post, delete_post, get_post, list_posts, login, register, update_post,
        },
    },
};

const CORS_MAX_AGE: usize = 3600;

mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    configure_logger();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let cors_allowrd_origins: Vec<String> = std::env::var("CORS_ALLOWED_ORIGINS")
        .expect("CORS_ALLOWED_ORIGINS must be set")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

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
    let grpc_service = BlogGrpcService::new(auth_service.clone(), blog_service.clone());

    let grpc_server = tonic::transport::Server::builder()
        .add_service(BlogServiceServer::with_interceptor(
            grpc_service,
            grpc_auth_interceptor(jwt_service.clone()),
        ))
        .serve(
            "0.0.0.0:50051"
                .parse()
                .expect("failed to parse SocketAddr for the grpc server"),
        );

    let http_server = HttpServer::new(move || {
        let auth_middleware = create_auth_middleware(jwt_service.clone());

        let mut cors = Cors::default();
        for url in &cors_allowrd_origins {
            cors = cors.allowed_origin(url);
        }
        cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .max_age(CORS_MAX_AGE);

        App::new()
            .wrap(cors)
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
    .bind("0.0.0.0:8080")?
    .run();

    // TODO() Consider graceful shutdown
    tokio::select! {
        res = http_server => {
            if let Err(e) = res {
                tracing::error!("HTTP server error: {}", e);
            }
        }
        res = grpc_server => {
            if let Err(e) = res {
                tracing::error!("gRPC server error: {}", e);
            }
        }
    }

    Ok(())
}
