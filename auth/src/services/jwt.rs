use crate::{
    application::{
        port::{Jwt, JwtPort},
        projection::User,
    },
    scalar::{Email, UserId},
};
use forgen::UnexpectedError;
use hmac::Hmac;
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
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

impl JwtPort for JwtService {
    fn sign(&self, user: &User) -> Result<Jwt, UnexpectedError> {
        let header = Header {
            algorithm: AlgorithmType::Hs384,
            ..Default::default()
        };
        let token = Token::new(header, UserDto::from(user))
            .sign_with_key(&self.key)
            .map_err(UnexpectedError::from)?;
        Ok(Jwt(token.as_str().to_string()))
    }

    fn verify(&self, token: &Jwt) -> Result<User, UnexpectedError> {
        let token: Token<Header, UserDto, _> = token
            .0
            .verify_with_key(&self.key)
            .map_err(UnexpectedError::from)?;
        let user = token.claims().clone();
        User::try_from(user)
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct UserDto {
    email: String,
    user_id: String,
}

impl TryFrom<UserDto> for User {
    type Error = UnexpectedError;

    fn try_from(dto: UserDto) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Some(Email::parse(dto.email).map_err(UnexpectedError::from)?),
            user_id: Some(UserId::parse(&dto.user_id).map_err(UnexpectedError::from)?),
        })
    }
}

impl From<&User> for UserDto {
    fn from(projection: &User) -> Self {
        Self {
            email: projection
                .email
                .as_ref()
                .map(|email| email.to_string())
                .unwrap_or_default(),
            user_id: projection
                .user_id
                .as_ref()
                .map(|user_id| user_id.to_string())
                .unwrap_or_default(),
        }
    }
}
