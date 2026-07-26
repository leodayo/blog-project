use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct CreatePostRequest {
    title: String,
    content: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdatePostRequest {
    title: String,
    content: String,
}
