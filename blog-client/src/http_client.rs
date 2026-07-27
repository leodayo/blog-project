use reqwest::{Client, StatusCode};
use serde_json::json;

use crate::{
    dto::{AuthResponse, ListPostsResponse, Post},
    error::BlogClientError,
};

pub struct HttpClient {
    base_url: String,
    reqwest: reqwest::Client,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_owned(),
            reqwest: Client::new(),
        }
    }

    pub fn request(&self) -> RequestBuilder<'_> {
        RequestBuilder {
            http: self,
            token: None,
        }
    }
}

pub struct RequestBuilder<'a> {
    http: &'a HttpClient,
    token: Option<String>,
}

impl<'a> RequestBuilder<'a> {
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
        let resp = self
            .http
            .reqwest
            .post(format!("{}/api/auth/register", self.http.base_url))
            .json(&json!({"username": username, "email": email, "password":password}))
            .send()
            .await?;

        process_response_status(resp.status())?;
        Ok(resp.json().await?)
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let resp = self
            .http
            .reqwest
            .post(format!("{}/api/auth/login", self.http.base_url))
            .json(&json!({"username": username, "password":password}))
            .send()
            .await?;

        process_response_status(resp.status())?;
        Ok(resp.json().await?)
    }

    pub async fn get_post(&self, id: i64) -> Result<Post, BlogClientError> {
        let mut req = self
            .http
            .reqwest
            .get(format!("{}/api/posts/{}", self.http.base_url, id));

        if let Some(t) = &self.token {
            req = req.bearer_auth(t);
        }

        let resp = req.send().await?;
        process_response_status(resp.status())?;
        Ok(resp.json().await?)
    }

    pub async fn list_posts(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<ListPostsResponse, BlogClientError> {
        let mut req = self
            .http
            .reqwest
            .get(format!("{}/api/posts", self.http.base_url))
            .query(&[("limit", limit), ("offset", offset)]);

        if let Some(t) = &self.token {
            req = req.bearer_auth(t);
        }

        let resp = req.send().await?;
        process_response_status(resp.status())?;
        Ok(resp.json().await?)
    }

    pub async fn create_post(&self, title: &str, content: &str) -> Result<Post, BlogClientError> {
        let Some(token) = &self.token else {
            return Err(BlogClientError::Unauthorized);
        };

        let resp = self
            .http
            .reqwest
            .post(format!("{}/api/posts", self.http.base_url))
            .bearer_auth(token)
            .json(&json!({ "title": title, "content": content }))
            .send()
            .await?;

        process_response_status(resp.status())?;
        Ok(resp.json().await?)
    }

    pub async fn update_post(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let Some(token) = &self.token else {
            return Err(BlogClientError::Unauthorized);
        };

        let resp = self
            .http
            .reqwest
            .put(format!("{}/api/posts/{}", self.http.base_url, id))
            .bearer_auth(token)
            .json(&json!({ "title": title, "content": content }))
            .send()
            .await?;

        process_response_status(resp.status())?;
        Ok(resp.json().await?)
    }

    pub async fn delete_post(&self, id: i64) -> Result<(), BlogClientError> {
        let Some(token) = &self.token else {
            return Err(BlogClientError::Unauthorized);
        };

        let resp = self
            .http
            .reqwest
            .delete(format!("{}/api/posts/{}", self.http.base_url, id))
            .bearer_auth(token)
            .send()
            .await?;

        process_response_status(resp.status())?;
        Ok(())
    }
}

fn process_response_status(status: StatusCode) -> Result<(), BlogClientError> {
    match status {
        StatusCode::NOT_FOUND => Err(BlogClientError::NotFound),
        StatusCode::UNAUTHORIZED => Err(BlogClientError::Unauthorized),
        StatusCode::FORBIDDEN => Err(BlogClientError::Forbidden),
        StatusCode::BAD_REQUEST => Err(BlogClientError::InvalidRequest),
        _ => Ok(()),
    }
}
