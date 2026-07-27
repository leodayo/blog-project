use blog_proto::{
    AuthResponse, CreatePostRequest, DeletePostRequest, GetPostRequest, ListPostsRequest,
    ListPostsResponse, LoginRequest, Post, RegisterRequest, UpdatePostRequest,
    blog_service_client::BlogServiceClient,
};
use tonic::{Request, metadata::MetadataValue, transport::Channel};

use crate::error::BlogClientError;

pub struct GrpcClient {
    inner: BlogServiceClient<Channel>,
}

impl GrpcClient {
    pub async fn connect(addr: &str) -> Result<Self, BlogClientError> {
        let channel = Channel::from_shared(addr.to_owned())?.connect().await?;

        Ok(Self {
            inner: BlogServiceClient::new(channel),
        })
    }

    pub fn request(&self) -> GrpcRequestBuilder {
        GrpcRequestBuilder {
            client: self.inner.clone(),
            token: None,
        }
    }
}

pub struct GrpcRequestBuilder {
    client: BlogServiceClient<Channel>,
    token: Option<String>,
}

impl GrpcRequestBuilder {
    pub fn with_auth(mut self, token: &str) -> Self {
        self.token = Some(token.to_owned());
        self
    }

    pub async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let req = RegisterRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };
        let resp = self.client.clone().register(req).await?;

        Ok(resp.into_inner())
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let req = LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        };
        let resp = self.client.clone().login(req).await?;

        Ok(resp.into_inner())
    }

    pub async fn get_post(&self, id: i64) -> Result<Post, BlogClientError> {
        let req = self.prepare_request(GetPostRequest { id });
        let resp = self.client.clone().get_post(req).await?;

        Ok(resp.into_inner())
    }

    pub async fn list_posts(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<ListPostsResponse, BlogClientError> {
        let page = (offset / limit) + 1;
        let req = self.prepare_request(ListPostsRequest { page, limit });
        let resp = self.client.clone().list_posts(req).await?;

        Ok(resp.into_inner())
    }

    pub async fn create_post(&self, title: &str, content: &str) -> Result<Post, BlogClientError> {
        let req = self.prepare_protected_request(CreatePostRequest {
            title: title.to_string(),
            content: content.to_string(),
        })?;
        let resp = self.client.clone().create_post(req).await?;

        Ok(resp.into_inner())
    }

    pub async fn update_post(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let req = self.prepare_protected_request(UpdatePostRequest {
            id,
            title: title.to_string(),
            content: content.to_string(),
        })?;
        let resp = self.client.clone().update_post(req).await?;

        Ok(resp.into_inner())
    }

    pub async fn delete_post(&self, id: i64) -> Result<(), BlogClientError> {
        let req = self.prepare_protected_request(DeletePostRequest { id })?;
        self.client.clone().delete_post(req).await?;

        Ok(())
    }

    fn prepare_request<T>(&self, message: T) -> Request<T> {
        let mut req = Request::new(message);
        if let Some(token) = &self.token {
            if let Ok(meta_val) = MetadataValue::try_from(format!("Bearer {}", token)) {
                req.metadata_mut().insert("authorization", meta_val);
            }
        }
        req
    }

    fn prepare_protected_request<T>(&self, message: T) -> Result<Request<T>, BlogClientError> {
        if self.token.is_none() {
            return Err(BlogClientError::Unauthorized);
        }
        Ok(self.prepare_request(message))
    }
}
