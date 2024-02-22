use super::projection::User;
use super::signal::Signal;
use crate::domain;
use crate::domain::scalar::*;
use forgen::*;

#[delegate]
pub trait EventStore {
    fn identify_by_email(&self, email: &Email) -> Result<Option<UserId>, UnexpectedError>;
    fn pull_by_user_id(&self, user_id: &UserId) -> Result<Vec<domain::Event>, UnexpectedError>;
    fn push(&self, user_id: &UserId, events: &[domain::Event]) -> Result<(), UnexpectedError>;
}

#[delegate]
pub trait UserRepository {
    fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError>;
    fn save(&self, user_id: &UserId, projection: &User) -> Result<(), UnexpectedError>;
}

#[delegate]
pub trait SignalBus {
    fn publish(&self, signal: Signal);
    fn subscribe<F>(&self, handler: F)
    where
        F: Fn(Signal) + Send + 'static;
}

#[delegate]
pub trait JwtPort {
    fn sign(&self, user: &User) -> Result<Jwt, UnexpectedError>;
    fn verify(&self, token: &Jwt) -> Result<User, UnexpectedError>;
}

pub struct Jwt(pub String);

pub trait LinkedInPort {
    fn sign_in(&self, code: &str) -> Result<LinkedInTokens, UnexpectedError>;
    fn get_email(&self, tokens: &LinkedInTokens) -> Result<Email, UnexpectedError>;
}

pub struct LinkedInTokens {
    pub access_token: String,
    pub refresh_token: String,
}
