use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{errors::AppError, models::Claims};

pub fn generate_jwt_token(claims: &Claims, secret: &str) -> Result<String, AppError> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}

pub fn validate_jwt_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    // In a real implementation, you would use a proper password hashing library like bcrypt
    // For now, just return the password as-is (NOT SECURE!)
    Ok(password.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    // In a real implementation, you would properly verify the hashed password
    // For now, just compare directly (NOT SECURE!)
    Ok(password == hash)
}