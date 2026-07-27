use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for blog_proto::User {
    fn from(user: User) -> Self {
        blog_proto::User {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: Option<String>,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Post> for blog_proto::Post {
    fn from(post: Post) -> Self {
        blog_proto::Post {
            id: post.id,
            title: post.title,
            content: post.content.unwrap_or_else(|| String::new()),
            author_id: post.author_id,
            created_at: Some(SystemTime::from(post.created_at).into()),
            updated_at: Some(SystemTime::from(post.updated_at).into()),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ListPostsResponse {
    pub posts: Vec<Post>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}
