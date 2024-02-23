use super::projection::User;
use super::signal::Signal;
use crate::domain;
use crate::domain::scalar::*;
use forgen::*;

#[delegate]
pub trait EventStore {
    fn identify_by_email(&self, email: &Email) -> Result<Option<UserId>, UnexpectedError>;
    fn pull_by_user_id(&self, user_id: &UserId) -> Result<Vec<domain::Event>, UnexpectedError>;
    fn push(&self, events: &[domain::Event]) -> Result<(), UnexpectedError>;
}

pub struct Jwt(pub String);

#[delegate]
pub trait JwtPort {
    fn sign(&self, user: &User) -> Result<Jwt, UnexpectedError>;
    fn verify(&self, token: &Jwt) -> Result<User, UnexpectedError>;
}

#[delegate]
pub trait LinkedInPort {
    fn sign_in(&self, code: &str) -> Result<LinkedInTokens, UnexpectedError>;
    fn get_email(&self, tokens: &LinkedInTokens) -> Result<Email, UnexpectedError>;
}

pub struct LinkedInTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[delegate]
pub trait SignalPub {
    fn publish(&self, signal: Signal);
}

#[delegate]
pub trait UserRepository {
    fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError>;
    fn save(&self, projection: &User) -> Result<(), UnexpectedError>;
}
