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
