use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;
use zeroize::Zeroizing;

use super::{AuthError, AuthResult};

pub fn hash_password(password: &str) -> AuthResult<String> {
    let password = Zeroizing::new(password.to_owned());
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AuthError::Other(format!("خطا در هش رمز: {e}")))
}

pub fn verify_password(password: &str, password_hash: &str) -> AuthResult<bool> {
    let password = Zeroizing::new(password.to_owned());
    let parsed = PasswordHash::new(password_hash)
        .map_err(|e| AuthError::Other(format!("هش نامعتبر: {e}")))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_roundtrip() {
        let hash = hash_password("Secret123!").unwrap();
        assert!(verify_password("Secret123!", &hash).unwrap());
        assert!(!verify_password("wrong-pass", &hash).unwrap());
    }
}
