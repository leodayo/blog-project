use std::sync::Arc;

use crate::{
    data::post_repository::PostRepository,
    domain::{error::AppError, post::Post},
    presentation::middleware::UserContext,
};

pub struct BlogService {
    post_repo: Arc<PostRepository>,
}

impl BlogService {
    pub fn new(post_repo: Arc<PostRepository>) -> Self {
        Self { post_repo }
    }

    pub async fn create_post(
        &self,
        user: UserContext,
        title: &str,
        content: &str,
    ) -> Result<Post, AppError> {
        let UserContext::Authenticated(user) = user else {
            return Err(AppError::Unauthenticated);
        };
        self.post_repo.create_post(user.id, title, content).await
    }

    pub async fn get_post(&self, id: i64) -> Result<Post, AppError> {
        self.post_repo.find_by_id(id).await
    }

    pub async fn get_posts(&self, limit: i64, offset: i64) -> Result<(Vec<Post>, i64), AppError> {
        self.post_repo.find_all(limit, offset).await
    }

    pub async fn update_post(
        &self,
        id: i64,
        user: UserContext,
        title: &str,
        content: &str,
    ) -> Result<Post, AppError> {
        let UserContext::Authenticated(user) = user else {
            return Err(AppError::Unauthenticated);
        };

        let target_post = self.get_post(id).await?;
        if user.id != target_post.author_id {
            return Err(AppError::Forbidden);
        }

        self.post_repo.update_post(id, title, content).await
    }

    pub async fn delete_post(&self, id: i64, user: UserContext) -> Result<(), AppError> {
        let UserContext::Authenticated(user) = user else {
            return Err(AppError::Unauthenticated);
        };

        let target_post = self.get_post(id).await?;
        if user.id != target_post.author_id {
            return Err(AppError::Forbidden);
        }

        self.post_repo.delete_post(id).await
    }
}
