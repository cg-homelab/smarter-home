use argon2::{
    password_hash::{rand_core::OsRng, Salt, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lib_models::{error::Error, HashedPassword};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// Define a struct to represent the claims in the JWT
#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String, // Subject (user ID or username)
    exp: usize,  // Expiration time
}

/// Function to generate a JWT
/// # Arguments
/// * `secret` - The secret key used to sign the JWT
/// * `sub` - The subject (user ID or username) to include in the JWT
/// # Returns
/// * A JWT string
/// # Example
/// ```rust
/// let secret = b"my_secret";
/// let token = generate_jwt(secret , "user123".to_string());
/// assert!(!token.is_empty());
/// ```
pub fn generate_jwt(secret: &[u8], sub: String) -> String {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600; // 1 hour expiration
    let claims = Claims {
        sub,
        exp: exp as usize,
    };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(secret);

    encode(&header, &claims, &encoding_key).unwrap()
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
