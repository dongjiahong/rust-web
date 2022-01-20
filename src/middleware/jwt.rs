use crate::common::error::Error;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT 鉴权 Token结构
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct JWTToken {
    // account id
    pub id: String,
    // account
    pub account: String,
    // permission set
    pub permission: Vec<String>,
    // role dis
    pub role_ids: Vec<String>,
    // expired
    pub exp: usize,
}

impl JWTToken {
    pub fn create_token(&self, secret: &str) -> Result<String, Error> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("JWTToken encode fail!")),
        };
    }

    pub fn verify(secret: &str, token: &str) -> Result<JWTToken, Error> {
        let validation = Validation {
            ..Validation::default()
        };
        return match decode::<JWTToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => return Err(Error::from("InvalidToken")),
                ErrorKind::ExpiredSignature => return Err(Error::from("ExpiredSignature")),
                ErrorKind::InvalidIssuer => return Err(Error::from("InvalidIssuer")),
                _ => return Err(Error::from("InvalidToken other errors")),
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rbatis::DateTimeNative;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_jwt() {
        let j = JWTToken {
            id: "1".to_string(),
            account: "988".to_string(),
            permission: vec![],
            role_ids: vec![],
            exp: DateTimeNative::now().timestamp_millis() as usize,
        };
        sleep(Duration::from_secs(5));
        let token = j.create_token("secret").unwrap();
        assert_eq!(JWTToken::verify("secret", &token).unwrap(), j);
    }
}
