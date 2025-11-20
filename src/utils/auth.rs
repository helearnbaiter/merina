use crate::utils::{AppError, AppResult};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

impl Claims {
    pub fn new(user_id: String, roles: Vec<String>, permissions: Vec<String>) -> Self {
        let now = Utc::now();
        let exp = now.checked_add_signed(Duration::seconds(3600)).unwrap(); // 1 hour default

        Claims {
            sub: user_id,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            roles,
            permissions,
        }
    }
}

pub fn generate_jwt_token(claims: &Claims, secret: &str) -> AppResult<String> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| AppError::InternalError(format!("Failed to encode JWT: {}", e)))?;

    Ok(token)
}

pub fn validate_jwt_token(token: &str, secret: &str) -> AppResult<Claims> {
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_exp = true;
    validation.validate_iat = true;
    validation.set_issuer(&["data-platform"]);
    validation.set_audience(&["users"]);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map_err(|e| {
        tracing::error!("JWT validation error: {}", e);
        AppError::AuthError("Invalid token".to_string())
    })?;

    Ok(token_data.claims)
}

pub fn generate_refresh_token(user_id: &str, secret: &str) -> AppResult<String> {
    let now = Utc::now();
    let exp = now.checked_add_signed(Duration::days(30)).unwrap(); // 30 days for refresh token

    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
        roles: vec!["refresh".to_string()],
        permissions: vec![],
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| AppError::InternalError(format!("Failed to encode refresh token: {}", e)))?;

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_validation() {
        let claims = Claims::new("user123".to_string(), vec!["user".to_string()], vec!["read".to_string()]);
        let secret = "test_secret";
        
        let token = generate_jwt_token(&claims, secret).unwrap();
        let decoded_claims = validate_jwt_token(&token, secret).unwrap();
        
        assert_eq!(claims.sub, decoded_claims.sub);
    }
}