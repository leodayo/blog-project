use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: i64,
    pub username: String,
}

#[derive(Debug, Clone)]
pub enum UserContext {
    Authenticated(AuthenticatedUser),
    Anonymous,
}
