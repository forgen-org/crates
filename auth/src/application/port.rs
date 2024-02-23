use super::projection::User;
use super::signal::Signal;
use crate::domain;
use crate::domain::scalar::*;
use forgen::*;

#[async_trait]
#[delegate]
pub trait EventStore {
    async fn identify_by_email(&self, email: &Email) -> Result<Option<UserId>, UnexpectedError>;
    async fn pull_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<domain::Event>, UnexpectedError>;
    async fn push(&self, events: &[domain::Event]) -> Result<(), UnexpectedError>;
}

pub struct Jwt(pub String);

#[async_trait]
#[delegate]
pub trait JwtPort {
    async fn sign(&self, user: &User) -> Result<Jwt, UnexpectedError>;
    async fn verify(&self, token: &Jwt) -> Result<User, UnexpectedError>;
}

#[async_trait]
#[delegate]
pub trait LinkedInPort {
    async fn sign_in(&self, code: &str) -> Result<LinkedInTokens, UnexpectedError>;
    async fn get_email(&self, tokens: &LinkedInTokens) -> Result<Email, UnexpectedError>;
}

pub struct LinkedInTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[async_trait]
#[delegate]
pub trait SignalPub {
    async fn publish(&self, signal: Signal);
}

#[async_trait]
#[delegate]
pub trait UserRepository {
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError>;
    async fn save(&self, projection: &User) -> Result<(), UnexpectedError>;
}
