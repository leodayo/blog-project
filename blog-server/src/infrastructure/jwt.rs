use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

const JWT_TTL_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Claims {
    pub user_id: i64,
    pub username: String,
    pub exp: i64,
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn generate_token(
        &self,
        user_id: i64,
        username: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let exp = (Utc::now() + Duration::hours(JWT_TTL_HOURS)).timestamp();

        let claims = Claims {
            user_id,
            username: username.to_owned(),
            exp,
        };

        jsonwebtoken::encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let validation = jsonwebtoken::Validation::default();
        jsonwebtoken::decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    const SECRET: &str = "test_secret_32_bytes_long_for_testing";

    fn get_service() -> JwtService {
        JwtService::new(SECRET)
    }

    #[test]
    fn test_generate_and_verify_valid() {
        let service = get_service();
        let user_id = 1;
        let username = "testuser";

        let token = service.generate_token(user_id, username).unwrap();
        let claims = service.verify_token(&token).unwrap();

        assert_eq!(claims.user_id, user_id);
        assert_eq!(claims.username, username);
        assert!(claims.exp > Utc::now().timestamp());
    }

    #[test]
    fn test_verify_invalid_token() {
        let service = get_service();
        let token = "invalid_token";
        let result = service.verify_token(token);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_expired_token() {
        let service = get_service();
        let exp = Utc::now().timestamp() - 3600;
        let claims = Claims {
            user_id: 1,
            username: "testuser".to_string(),
            exp,
        };
        let token =
            jsonwebtoken::encode(&Header::default(), &claims, &service.encoding_key).unwrap();
        let result = service.verify_token(&token);
        assert!(result.is_err());
    }
}
