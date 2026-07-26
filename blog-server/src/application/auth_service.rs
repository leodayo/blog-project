use std::sync::Arc;

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::{
    data::user_repository::UserRepository,
    domain::{error::AppError, user::User},
    infrastructure::jwt::JwtService,
};

pub struct AuthService {
    user_repository: Arc<UserRepository>,
    jwt_service: Arc<JwtService>,
}

impl AuthService {
    pub fn new(user_repository: Arc<UserRepository>, jwt_service: Arc<JwtService>) -> Self {
        Self {
            user_repository,
            jwt_service,
        }
    }

    pub async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<(String, User), AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                tracing::error!("Password hashing failed: {}", e);
                AppError::InternalError
            })?
            .to_string();

        let user = self
            .user_repository
            .create_user(username, email, &password_hash)
            .await?;

        let token = self
            .jwt_service
            .generate_token(user.id, &user.username)
            .map_err(|e| {
                tracing::error!("Failed to generate JWT for user {}: {}", user.id, e);
                AppError::InternalError
            })?;

        Ok((token, user))
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<(String, User), AppError> {
        let user = self
            .user_repository
            .find_by_username(username)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|e| {
            tracing::error!("Failed to parse password hash: {}", e);
            AppError::InternalError
        })?;
        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|e| {
                tracing::error!("Failed to verify password for user {}: {}", user.id, e);
                AppError::InternalError
            })?;

        let token = self
            .jwt_service
            .generate_token(user.id, &user.username)
            .map_err(|e| {
                tracing::error!("Failed to generate JWT for user {}: {}", user.id, e);
                AppError::InternalError
            })?;

        Ok((token, user))
    }
}
