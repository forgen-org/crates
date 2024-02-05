use framework::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl ToString for UserId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn parse<T>(value: T) -> Result<Self, EmailError>
    where
        T: ToString,
    {
        let value = value.to_string();
        let regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();
        if !regex.is_match(&value) {
            return Err(EmailError::InvalidEmail);
        }
        Ok(Email(value))
    }
}

impl ToString for Email {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Invalid email")]
    InvalidEmail,
}

#[derive(Serialize, Deserialize)]
pub struct Password(String);

impl Password {
    pub fn parse<T>(value: T) -> Result<Self, PasswordError>
    where
        T: ToString,
    {
        let value = value.to_string();
        if value.len() < 8 || value.len() > 20 {
            return Err(PasswordError::InvalidLength);
        }
        Ok(Password(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Password must be between 8 and 20 characters")]
    InvalidLength,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct PasswordHash([u8; 32]);

impl From<Password> for PasswordHash {
    fn from(password: Password) -> Self {
        Self::from(&password)
    }
}

impl From<&Password> for PasswordHash {
    fn from(password: &Password) -> Self {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&password.0);
        let value = hasher.finalize();
        Self(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email() {
        let email = Email::parse("email@example.com");
        assert!(matches!(email, Ok(_)));

        let invalid_email = Email::parse("invalid_email");
        assert!(matches!(invalid_email, Err(EmailError::InvalidEmail)));
    }

    #[test]
    fn test_password() {
        let password: Result<Password, PasswordError> = Password::parse("password");
        assert!(matches!(password, Ok(_)));

        let too_short = Password::parse("u");
        assert!(matches!(too_short, Err(PasswordError::InvalidLength)));

        let too_long = Password::parse("username_that_is_too_long");
        assert!(matches!(too_long, Err(PasswordError::InvalidLength)));
    }

    #[test]
    fn test_password_hash() {
        let password = Password::parse("password").unwrap();
        assert_eq!(
            PasswordHash::from(&password).0,
            [
                94, 136, 72, 152, 218, 40, 4, 113, 81, 208, 229, 111, 141, 198, 41, 39, 115, 96,
                61, 13, 106, 171, 189, 214, 42, 17, 239, 114, 29, 21, 66, 216
            ]
        );
    }
}
