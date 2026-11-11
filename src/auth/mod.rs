mod db;
mod password;
mod session;
mod validation;

pub use db::AuthStore;
pub use session::{Session, SessionManager};
pub use validation::{password_strength, PasswordStrength};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("{0}")]
    Validation(String),
    #[error("نام کاربری یا رمز عبور اشتباه است")]
    InvalidCredentials,
    #[error("حساب موقتاً قفل شده است. لطفاً کمی بعد تلاش کنید")]
    AccountLocked,
    #[error("این نام کاربری قبلاً ثبت شده است")]
    UsernameTaken,
    #[error("این ایمیل قبلاً ثبت شده است")]
    EmailTaken,
    #[error("نشست منقضی شده است. دوباره وارد شوید")]
    SessionExpired,
    #[error("خطای پایگاه‌داده: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("{0}")]
    Other(String),
}

pub type AuthResult<T> = Result<T, AuthError>;
