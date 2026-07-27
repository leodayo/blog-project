use std::sync::Arc;

use blog_proto::blog_service_server::BlogService;
use blog_proto::{
    AuthResponse, CreatePostRequest, DeletePostRequest, GetPostRequest, ListPostsRequest,
    ListPostsResponse, LoginRequest, Post, RegisterRequest, UpdatePostRequest, User,
};
use tonic::{Request, Response, Status};

use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService as AppBlogService;
use crate::infrastructure::jwt::JwtService;
use crate::presentation::grpc_middleware::TonicAuthExt;

pub struct BlogGrpcService {
    auth_service: Arc<AuthService>,
    blog_service: Arc<AppBlogService>,
    jwt_service: Arc<JwtService>,
}

impl BlogGrpcService {
    pub fn new(
        auth_service: Arc<AuthService>,
        blog_service: Arc<AppBlogService>,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self {
            auth_service,
            blog_service,
            jwt_service,
        }
    }
}

#[tonic::async_trait]
impl BlogService for BlogGrpcService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        let (token, user) = self
            .auth_service
            .register(&req.username, &req.email, &req.password)
            .await?;

        Ok(Response::new(AuthResponse {
            token,
            user: Some(user.into()),
        }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        let (token, user) = self
            .auth_service
            .login(&req.username, &req.password)
            .await?;

        Ok(Response::new(AuthResponse {
            token,
            user: Some(user.into()),
        }))
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<Post>, Status> {
        let user = request.require_authenticated()?;
        let req = request.into_inner();

        let post = self
            .blog_service
            .create_post(user, &req.title, &req.content)
            .await?;

        Ok(Response::new(post.into()))
    }

    async fn get_post(&self, request: Request<GetPostRequest>) -> Result<Response<Post>, Status> {
        let req = request.into_inner();
        let post = self.blog_service.get_post(req.id).await?;

        Ok(Response::new(post.into()))
    }
    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> Result<Response<Post>, Status> {
        let user = request.require_authenticated()?;
        let req = request.into_inner();

        let post = self
            .blog_service
            .update_post(req.id, user, &req.title, &req.content)
            .await?;

        Ok(Response::new(post.into()))
    }
    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<()>, Status> {
        let user = request.require_authenticated()?;
        let req = request.into_inner();

        self.blog_service.delete_post(req.id, user).await?;

        Ok(Response::new(()))
    }
    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        let req = request.into_inner();
        let limit = req.limit;
        let offset = (req.page - 1) * req.limit;

        let (posts, total) = self.blog_service.get_posts(limit, offset).await?;
        let proto_posts: Vec<blog_proto::Post> = posts.into_iter().map(Into::into).collect();

        Ok(Response::new(ListPostsResponse {
            posts: proto_posts,
            total: total,
        }))
    }
}
