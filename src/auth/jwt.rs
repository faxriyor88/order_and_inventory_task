use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use jsonwebtoken::errors::Error;
use jsonwebtoken::{encode, decode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret)
        }
    }
}

pub fn verify_token(token: &str, key: &DecodingKey) -> Result<Claims, Error> {
    let data: TokenData<Claims> = decode(token, key, &Validation::default())?;
    Ok(data.claims)
}

pub fn create_token(user_id: i64, key: &EncodingKey) -> Result<String, Error> {
    let exp = (Utc::now() + Duration::minutes(15)).timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };

    encode(&Header::default(), &claims, key)
}