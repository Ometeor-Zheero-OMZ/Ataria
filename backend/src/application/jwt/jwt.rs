use axum::http::HeaderMap;
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration};
use jsonwebtoken::errors::Error as JWTError;
use lambda_http::tracing::error;

use crate::application::helpers::message::AUTH_MSG;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: i32,
    pub sub: String,
    pub exp: usize,
}

pub fn create_token(email: &str, id: &i32) -> Result<String, JWTError> {
    // トークンの有効期限 10日
    let days = 60 * 60 * 24 * 10;
    let expiration = SystemTime::now() + Duration::from_secs(days);
    let claims = Claims {
        id: id.to_owned(),
        sub: email.to_owned(),
        exp: expiration.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref())
    )
}

pub fn decode_token(token: &str) -> Result<TokenData<Claims>, JWTError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256)
    )
}

pub fn verify(headers: &HeaderMap)  -> Result<Claims, String> {
    // リクエストヘッダーから Bearer トークンを抽出できる場合
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            // 接頭辞の "Bearer" を抽出
            let parts: Vec<&str> = auth_str.split_whitespace().collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                let token = parts[1];
                // トークンを認証し、ユーザー情報をデコード
                match decode_token(token) {
                    Ok(user_info) => {
                        return Ok(user_info.claims);
                    },
                    Err(error) => {
                        error!("[jwt] - [verify] error = {}", error);
                        return Err(error.to_string());
                    }
                }
            }
        }
    }
    Err(AUTH_MSG
        .get("TOKEN_NOT_FOUND_IN_REQUEST_HEADER_MSG")
        .unwrap_or(&"")
        .to_string())
}