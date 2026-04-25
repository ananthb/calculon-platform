use serde::{Deserialize, Serialize};
use worker::*;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,
    pub email: String,
}

pub struct Auth {
    pub secret: String,
}

impl Auth {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn create_session(&self, user_id: &str, email: &str) -> Result<String> {
        let exp = Utc::now() + Duration::days(30);
        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp() as usize,
            email: email.to_string(),
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.secret.as_bytes()))
            .map_err(|e| Error::from(format!("JWT encode error: {e}")))
    }

    pub fn verify_session(&self, token: &str) -> Result<Claims> {
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        ).map_err(|e| Error::from(format!("JWT decode error: {e}")))?;

        Ok(token_data.claims)
    }
}

pub mod oauth {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct OAuthUser {
        pub id: String,
        pub email: String,
        pub name: Option<String>,
        pub avatar_url: Option<String>,
        pub provider: String,
    }

    // Google, GitHub, Facebook OAuth logic would go here
    // For now, we'll just provide types for them.
}
