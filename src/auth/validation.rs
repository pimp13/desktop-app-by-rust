use regex::Regex;
use std::sync::OnceLock;

use super::{AuthError, AuthResult};

fn email_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)^[a-z0-9._%+\-]+@[a-z0-9.\-]+\.[a-z]{2,}$").unwrap())
}

fn username_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[a-zA-Z0-9_]{3,32}$").unwrap())
}

pub fn validate_username(username: &str) -> AuthResult<()> {
    let username = username.trim();
    if username.is_empty() {
        return Err(AuthError::Validation("نام کاربری الزامی است".into()));
    }
    if !username_re().is_match(username) {
        return Err(AuthError::Validation(
            "نام کاربری باید ۳ تا ۳۲ کاراکتر و فقط حروف، عدد و _ باشد".into(),
        ));
    }
    Ok(())
}

pub fn validate_email(email: &str) -> AuthResult<()> {
    let email = email.trim();
    if email.is_empty() {
        return Err(AuthError::Validation("ایمیل الزامی است".into()));
    }
    if !email_re().is_match(email) {
        return Err(AuthError::Validation("فرمت ایمیل معتبر نیست".into()));
    }
    Ok(())
}

pub fn validate_password(password: &str) -> AuthResult<()> {
    if password.len() < 8 {
        return Err(AuthError::Validation(
            "رمز عبور باید حداقل ۸ کاراکتر باشد".into(),
        ));
    }
    if password.len() > 128 {
        return Err(AuthError::Validation(
            "رمز عبور بیش از حد طولانی است".into(),
        ));
    }
    let has_letter = password.chars().any(|c| c.is_ascii_alphabetic());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    if !has_letter || !has_digit {
        return Err(AuthError::Validation(
            "رمز عبور باید شامل حرف و عدد باشد".into(),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordStrength {
    Weak,
    Fair,
    Good,
    Strong,
}

impl PasswordStrength {
    pub fn label_fa(self) -> &'static str {
        match self {
            Self::Weak => "ضعیف",
            Self::Fair => "متوسط",
            Self::Good => "خوب",
            Self::Strong => "قوی",
        }
    }

    pub fn score(self) -> f32 {
        match self {
            Self::Weak => 0.25,
            Self::Fair => 0.5,
            Self::Good => 0.75,
            Self::Strong => 1.0,
        }
    }
}

pub fn password_strength(password: &str) -> PasswordStrength {
    let mut score = 0;
    if password.len() >= 8 {
        score += 1;
    }
    if password.len() >= 12 {
        score += 1;
    }
    if password.chars().any(|c| c.is_ascii_lowercase())
        && password.chars().any(|c| c.is_ascii_uppercase())
    {
        score += 1;
    }
    if password.chars().any(|c| c.is_ascii_digit()) {
        score += 1;
    }
    if password.chars().any(|c| !c.is_ascii_alphanumeric()) {
        score += 1;
    }

    match score {
        0..=1 => PasswordStrength::Weak,
        2 => PasswordStrength::Fair,
        3..=4 => PasswordStrength::Good,
        _ => PasswordStrength::Strong,
    }
}
