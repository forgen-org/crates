use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Validate, Serialize, Deserialize)]
pub struct Username {
    #[validate(length(min = 3, max = 20))]
    value: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct Email {
    #[validate(email)]
    value: String,
}

impl Email {
    pub fn parse(email: &str) -> Result<Self, validator::ValidationErrors> {
        let email = Self {
            value: email.to_string(),
        };
        email.validate()?;
        Ok(email)
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Serialize, Deserialize)]
pub struct Password(pub String);

#[derive(Serialize, Deserialize)]
pub struct PasswordHash([u8; 32]);

impl From<Password> for PasswordHash {
    fn from(password: Password) -> Self {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(password.0);
        let value = hasher.finalize();
        Self(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash() {
        let password = Password("password".to_string());
        assert_eq!(
            PasswordHash::from(password).0,
            [
                94, 136, 72, 152, 218, 40, 4, 113, 81, 208, 229, 111, 141, 198, 41, 39, 115, 96,
                61, 13, 106, 171, 189, 214, 42, 17, 239, 114, 29, 21, 66, 216
            ]
        );
    }
}
