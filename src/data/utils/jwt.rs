use std::env::current_dir;

use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(60);
    now += expires_in;
    let exp = now.timestamp() as usize;

    let claims = Claims {
        exp: exp,
        iat
    };

    let secret = get_jwt_secret();

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())) 
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid(token: &str) -> Result<bool, StatusCode> {
    // verify(signature, message, key, algorithm)
    let secret = get_jwt_secret();
    let key = DecodingKey::from_secret(secret.as_bytes());

    decode::<Claims>(token, 
                    &key, 
                    &Validation::new(jsonwebtoken::Algorithm::HS256))
                .map_err(|_err| {
                    match _err.kind() {
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
                        _ => StatusCode::INTERNAL_SERVER_ERROR
                    }
                })?;
    Ok(true)
}

fn get_jwt_secret() -> String {
    let env_path = format!("{}/src/data/.env",current_dir().unwrap().display());
    dotenv::from_path(env_path).ok();
    std::env::var("JWT_SECRET").unwrap()
}