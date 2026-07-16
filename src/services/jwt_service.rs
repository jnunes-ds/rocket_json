use rocket::serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, decode, encode, DecodingKey, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

const SECRET_KEY: &str = "your_secret_key";

pub fn generate_jwt_token(admin_id: u32) -> String {
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: admin_id.to_owned().to_string(),
        exp: expiration_time as usize,
    };

    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(SECRET_KEY.as_ref())
    ).unwrap()
}

pub fn verify_token(token: &str) -> bool {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY.as_ref()),
        &Validation::default()
    ).is_ok()
}