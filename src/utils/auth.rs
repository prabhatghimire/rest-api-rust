use bcrypt::hash;
use std::env;

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: Option<String>, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: Option<usize>, // Optional. Issued at (as UTC timestamp)
    iss: Option<String>, // Optional. Issuer
    nbf: Option<usize>, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

pub fn generate_token(user_id: String, expiration_seconds: usize) -> String {
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not set in .env");
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + expiration_seconds;

    let claims = Claims {
        aud: None,
        sub: user_id,
        exp,
        iat: None,
        iss: None,
        nbf: None,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .expect("Failed to generate token")
}

// pub fn decode_token(token: String) -> Claims {
//     let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not set in .env");
//     let token_data = decode::<Claims>(
//         &token,
//         &DecodingKey::from_secret(secret_key.as_ref()),
//         &Validation::default(),
//     )
//     .expect("Failed to decode token");
//     token_data.claims
// }

// Hash a password using argon2
pub fn hash_password(password: String) -> String {
    let hashed_password = hash(password, 12).unwrap();

    println!("Hashed Passwosard: {}", hashed_password);
    hashed_password
}

// // Verify a password using argon2
// fn verify_password(hash: &str, password: &str) -> bool {
// Verify the password
// let is_valid = verify(password, &hashed_password).unwrap();
//     argon2::verify_encoded(hash, password.as_bytes()).unwrap_or(false)
// }
