use std::time::SystemTime;

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
            // TODO() feels dirty to convert DateTime into SystemTime first
            // just to convert it into prost_types::Timestamp right after.
            // I do realize it's not good performance-wise and it would be better
            // to implement a direct conversion. But I'll use this approach for now
            // and improve later when/if needed.
            //
            // UPD: on a second thought, it doesn't look that bad, if
            // we consider the build with --release the compiler will likely
            // inline everything and make it just a direct copy,
            // no alloc, no expensive compute. So I think I'll leave it as is until
            // proven wrong.
            created_at: Some(SystemTime::from(post.created_at).into()),
            updated_at: Some(SystemTime::from(post.updated_at).into()),
        }
    }
}
