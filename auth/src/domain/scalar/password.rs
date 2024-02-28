use forgen::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Password(#[serde(deserialize_with = "Password::deserialize")] String);

impl Password {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Password::parse(value)
            .map(|Password(value)| value)
            .map_err(serde::de::Error::custom)
    }

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

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Password must be between 8 and 20 characters")]
    InvalidLength,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password() {
        let password: Result<Password, PasswordError> = Password::parse("password");
        assert!(matches!(password, Ok(_)));

        let too_short = Password::parse("u");
        assert!(matches!(too_short, Err(PasswordError::InvalidLength)));

        let too_long = Password::parse("username_that_is_too_long");
        assert!(matches!(too_long, Err(PasswordError::InvalidLength)));
    }
}
