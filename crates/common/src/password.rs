use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Hashes a plaintext password using Argon2id with a freshly generated
/// random salt. The resulting string encodes the algorithm, salt, and
/// hash together (PHC string format) — safe to store directly in
/// `users.password_hash`.
pub fn hash_password(plain: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(plain.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

/// Verifies a plaintext password against a previously stored Argon2
/// hash. Returns `false` on any parse or verification failure — a
/// malformed stored hash should never be distinguishable from a wrong
/// password to the caller.
pub fn verify_password(plain: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => Argon2::default().verify_password(plain.as_bytes(), &parsed_hash).is_ok(),
        Err(_) => false,
    }
}