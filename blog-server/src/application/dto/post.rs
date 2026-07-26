use serde::{Deserialize, Serialize};

use crate::domain::post::Post;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ListPostsResponse {
    pub posts: Vec<Post>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}
