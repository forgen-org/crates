use crate::application::auth_port::*;
use framework::*;
use hmac::Hmac;
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use sha2::{digest::KeyInit, Sha384};

pub struct JwtService {
    key: Hmac<Sha384>,
}

impl JwtService {
    pub fn new(secret_key: &str) -> Self {
        Self {
            key: Hmac::new_from_slice(secret_key.as_bytes()).unwrap(),
        }
    }
}

#[async_trait]
impl JwtPort for JwtService {
    async fn sign(&self, user: &User) -> Result<Jwt, ServiceError> {
        let header = Header {
            algorithm: AlgorithmType::Hs384,
            ..Default::default()
        };
        let token = Token::new(header, user)
            .sign_with_key(&self.key)
            .map_err(ServiceError::from)?;
        Ok(Jwt(token.as_str().to_string()))
    }
    async fn verify(&self, token: &Jwt) -> Result<User, ServiceError> {
        let token: Token<Header, User, _> = token
            .0
            .verify_with_key(&self.key)
            .map_err(ServiceError::from)?;
        let user = token.claims();
        Ok(user.clone())
    }
}

impl From<jwt::error::Error> for ServiceError {
    fn from(error: jwt::error::Error) -> Self {
        ServiceError::UnknownError(format!("JwtService: {:?}", error))
    }
}