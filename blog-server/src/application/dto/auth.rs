use serde::{Deserialize, Serialize};

use crate::domain::user::User;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}
