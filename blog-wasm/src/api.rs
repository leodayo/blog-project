use gloo_net::http::{Method, RequestBuilder};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    dto::{
        AuthResponse, CreatePostRequest, ListPostsResponse, LoginRequest, Post, RegisterRequest,
        UpdatePostRequest,
    },
    error::ApiError,
};

// const API_PATH: &str = "/api";
const API_PATH: &str = "http://localhost:8080/api";

async fn request<T, B>(
    method: Method,
    path: &str,
    body: Option<B>,
    token: Option<&str>,
) -> Result<T, ApiError>
where
    T: DeserializeOwned,
    B: Serialize,
{
    let url = format!("{}{}", API_PATH, path);
    let mut req = RequestBuilder::new(&url).method(method);

    if let Some(token) = token {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }

    let resp = if let Some(body) = body {
        req.json(&body)?.send().await?
    } else {
        req.send().await?
    };

    if !resp.ok() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(ApiError::Http { status, body });
    }

    resp.json::<T>().await.map_err(ApiError::from)
}

pub async fn register(
    username: &str,
    email: &str,
    password: &str,
) -> Result<AuthResponse, ApiError> {
    let body = RegisterRequest {
        username: username.to_string(),
        email: email.to_string(),
        password: password.to_string(),
    };
    request(Method::POST, "/auth/register", Some(&body), None).await
}

pub async fn login(username: &str, password: &str) -> Result<AuthResponse, ApiError> {
    let body = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };
    request(Method::POST, "/auth/login", Some(&body), None).await
}

pub async fn list_posts(limit: i64, offset: i64) -> Result<ListPostsResponse, ApiError> {
    let path = format!("/posts?limit={}&offset={}", limit, offset);
    request(Method::GET, &path, None::<()>, None).await
}

pub async fn create_post(title: &str, content: &str, token: &str) -> Result<Post, ApiError> {
    let body = CreatePostRequest {
        title: title.to_string(),
        content: content.to_string(),
    };
    request(Method::POST, "/posts", Some(&body), Some(token)).await
}

pub async fn update_post(
    id: i64,
    title: &str,
    content: &str,
    token: &str,
) -> Result<Post, ApiError> {
    let body = UpdatePostRequest {
        title: title.to_string(),
        content: content.to_string(),
    };
    let path = format!("/posts/{}", id);
    request(Method::PUT, &path, Some(&body), Some(token)).await
}

pub async fn delete_post(id: i64, token: &str) -> Result<(), ApiError> {
    let path = format!("/posts/{}", id);
    let _: String = request(Method::DELETE, &path, None::<()>, Some(token)).await?;

    Ok(())
}
