use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lib_models::{error::Error, HashedPassword};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap();
    Keys::new(secret.as_bytes())
});

// encoding/decoding keys - set in the static `once_cell`
struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

// Define a struct to represent the claims in the JWT
#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID or username)
    pub exp: usize,  // Expiration time
}

// implement FromRequestParts for Claims (the JWT struct)
// FromRequestParts allows us to use Claims without consuming the request
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::InvalidToken)?;
        tracing::debug!("Extracted bearer token: {}", bearer.token());
        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &KEYS.decoding,
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| {
            tracing::error!("JWT decoding error: {:?}", err);
            Error::InvalidToken
        })?;

        Ok(token_data.claims)
    }
}

pub fn generate_base64_token() -> String {
    let random_bytes = uuid::Uuid::new_v4().as_bytes().to_vec();
    BASE64_URL_SAFE_NO_PAD.encode(&random_bytes)
}

/// Function to generate a JWT
/// # Arguments
/// * `secret` - The secret key used to sign the JWT
/// * `sub` - The subject (user ID or username) to include in the JWT
/// # Returns
/// * A JWT string
/// # Example
/// ```rust
/// use lib_utils::crypto::{generate_jwt, validate_jwt};
///
/// let token = generate_jwt("user123".to_string());
/// assert!(!token.is_empty());
/// ```
pub fn generate_jwt(sub: String, indefenant: bool) -> String {
    let exp = match indefenant {
        true => usize::MAX,
        false => {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize
                + 3600 // 1 hour expiration
        }
    };

    let claims = Claims { sub, exp };

    encode(&Header::default(), &claims, &KEYS.encoding).unwrap()
}

/// Function to validate a JWT
/// # Arguments
/// * `secret` - The secret key used to decode the JWT
/// * `token` - The JWT string to validate
/// # Returns
/// * `Ok(Claims)` if the token is valid, or an `Error` if it is not
/// # Errors
/// * `jsonwebtoken::errors::Error` if the token is invalid or expired
/// # Example
/// ```rust
/// use lib_utils::crypto::{generate_jwt, validate_jwt};
///
/// let secret = b"my_secret";
/// let token = generate_jwt(secret, "user123".to_string());
/// let claims = validate_jwt(secret, &token).unwrap();
/// assert_eq!(claims.sub, "user123");
/// ```
pub fn validate_jwt(secret: &[u8], token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(secret);
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(token, &decoding_key, &validation).map(|data| data.claims)
}

/// Hash a password using Argon2
/// # Arguments
/// * `password` - The plaintext password to hash
/// # Returns
/// * `Ok(HashedPassword)` containing the hashed password and salt, or an `Error` if hashing fails
/// # Errors
/// * `Error::CryptoHashError` if there was an error during hashing
pub fn hash_password(password: &str) -> Result<HashedPassword, Error> {
    let salt_str = SaltString::generate(&mut OsRng);
    let salt = salt_str.as_salt();
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), salt)
        .map_err(|_| Error::CryptoHashError)?
        .to_string();

    Ok(HashedPassword {
        hash: password_hash,
        salt: salt.as_str().to_string(),
    })
}

/// Verify a password against a stored hash
/// # Arguments
/// * `password` - The plaintext password to verify
/// * `hashed_password` - The stored hashed password
/// # Returns
/// * `Ok(true)` if the password matches, `Ok(false)` if it doesn't, or an `Error` if there was an issue during verification
/// # Errors
/// * `Error::CryptoHashError` if there was an error parsing the hash or during verification
/// # Example
/// ```rust
/// use lib_utils::crypto::{hash_password, verify_password};
///
/// let password = "my_secure_password";
/// let hashed = hash_password(password).unwrap();
/// let is_valid = verify_password(password, &hashed.hash).unwrap();
/// assert!(is_valid);
/// let is_invalid = verify_password("wrong_password", &hashed.hash).unwrap();
/// assert!(!is_invalid);
/// let is_error = verify_password(password, "invalid_hash");
/// assert!(is_error.is_err());
/// ```
pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, Error> {
    let parsed_hash =
        argon2::PasswordHash::new(hashed_password).map_err(|_| Error::CryptoHashError)?;
    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false), // Password mismatch
        Err(_) => Err(Error::CryptoHashError),                    // Other errors
    }
}
