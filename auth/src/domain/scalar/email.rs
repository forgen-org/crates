use framework::*;
use regex::Regex;

#[derive(Clone)]
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

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Invalid email")]
    InvalidEmail,
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
}
