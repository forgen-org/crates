use super::password::Password;

#[derive(Clone, PartialEq)]
pub struct PasswordHash(pub [u8; 32]);

impl From<Password> for PasswordHash {
    fn from(password: Password) -> Self {
        Self::from(&password)
    }
}

impl From<&Password> for PasswordHash {
    fn from(password: &Password) -> Self {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(password.as_str());
        let value = hasher.finalize();
        Self(value.into())
    }
}

impl PasswordHash {
    pub fn verify(&self, password: &Password) -> bool {
        let hash = PasswordHash::from(password);
        self.0 == hash.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
