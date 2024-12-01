use serde::Deserialize;
use validator::{Validate, ValidationError};
use regex::Regex;

#[derive(Deserialize, Debug, Validate)]
pub struct SignupRequest {
    pub name: String,
    #[validate(email(message = "Invalid email address"))]
    #[validate(length(max = 319, message = "Email address too long"))]
    #[validate(custom(function = "validate_email"))]
    pub email: String,
    #[validate(length(max = 127, message = "Password too long"))]
    #[validate(custom(function = "validate_password"))]
    pub password: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct LoginRequest {
    pub name: String,
    #[validate(email(message = "Invalid email address"))]
    #[validate(length(max = 319, message = "Email address too long"))]
    #[validate(custom(function = "validate_email"))]
    pub email: String,
    #[validate(length(max = 127, message = "Password too long"))]
    #[validate(custom(function = "validate_password"))]
    pub password: String,
}

// #[derive(Deserialize, Debug)]
// pub struct VerificationRequest {
//     pub token: String
// }

/// Eメールバリデーター
/// 
/// # 引数
/// 
/// * `password` - パスワード
/// 
/// # 戻り値
/// 
/// * `Result<(), ValidationError>`
fn validate_email(email: &str) -> Result<(), ValidationError> {
    // 無効な特殊文字を含む正規表現パターン
    let invalid_special_chars_regex = Regex::new(r"[!#$%^&*()+=[\/]{}]|<>;:?`~").unwrap();

    // メールアドレスに @ が含まれているか確認
    let at_index = email.find('@').ok_or_else(|| ValidationError::new("email_missing_at_sign"))?;
    
    // ドメイン部分を取得
    let domain_part = &email[at_index + 1..];

    // ドメイン部分に無効な特殊文字が含まれているか確認
    if invalid_special_chars_regex.is_match(domain_part) {
        return Err(ValidationError::new("email_contains_invalid_special_characters"));
    }

    // ドメイン部分に有効な特殊文字が含まれているか確認
    let valid_special_chars = ['.', '_', '-'];
    if !domain_part.chars().any(|c| valid_special_chars.contains(&c)) {
        return Err(ValidationError::new("email_missing_special_character_in_domain"));
    }

    // 有効なドメインを定義
    let valid_domains = ["gmail.com", "yahoo.com", "outlook.com"];

    // ドメイン部分が有効なドメインで終わっているか確認
    if !valid_domains.iter().any(|&valid_domain| domain_part.ends_with(valid_domain)) {
        return Err(ValidationError::new("email_invalid_domain"));
    }

    Ok(())
}


/// パスワードバリデーター
/// 
/// # 引数
/// 
/// * `password` - パスワード
/// 
/// # 戻り値
/// 
/// * `Result<(), ValidationError>`
fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("password_too_short"));
    }

    if !password.chars().any(|c| c.is_digit(10)) {
        return Err(ValidationError::new("password_no_digit"));
    }

    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::new("password_no_uppercase"));
    }

    Ok(())
}