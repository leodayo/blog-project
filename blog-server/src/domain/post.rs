use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
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
            created_at: Some(post.created_at.into()),
            updated_at: Some(post.updated_at.into()),
        }
    }
}
