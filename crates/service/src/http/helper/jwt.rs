use crate::error::ErrorKind;
use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});
/// jwt auth
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// email
    pub email: String,
    pub exp: i64,
}
impl Claims {
    pub fn new(email: String) -> Self {
        let iat = chrono::Utc::now();
        let exp = iat + chrono::Duration::hours(24);

        Self {
            email,
            exp: chrono::DateTime::timestamp(&exp),
        }
    }
    fn decode(token: &str) -> jsonwebtoken::errors::Result<jsonwebtoken::TokenData<Claims>> {
        decode::<Claims>(token, &KEYS.decoding, &Validation::default())
    }
    pub fn generate_jwt_token(&self) -> jsonwebtoken::errors::Result<String> {
        // Create the authorization token
        jsonwebtoken::encode(&Header::default(), self, &KEYS.encoding)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ErrorKind;
    
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|err| {
                    tracing::error!(?err);
                    ErrorKind::Unauthorized
                })?;
        // Decode the user data
        let token_data = Claims::decode(bearer.token())?;

        Ok(token_data.claims)
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims() {
        std::env::set_var("JWT_SECRET", "JWT_SECRET");
        let email = "email@eamil.com".to_string();
        let claims = Claims::new(email.clone());
        let jwt = claims.generate_jwt_token().unwrap();
        let decode = Claims::decode(&jwt).unwrap();

        assert_eq!(decode.claims.email, email)
    }
}
