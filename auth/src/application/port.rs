use std::pin::Pin;

use super::event::Event;
use super::projection::User;
use super::scalar::TransactionId;
use crate::domain;
use crate::domain::scalar::*;
use framework::*;
use futures::Stream;

pub type EventStream = Pin<Box<dyn Stream<Item = (Vec<Event>, Option<TransactionId>)> + Send>>;

#[async_trait]
#[delegate]
pub trait EventBus {
    async fn publish(&self, events: Vec<Event>, transaction_id: Option<TransactionId>) -> ();
    fn subscribe(&self) -> EventStream;
}

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

#[async_trait]
#[delegate]
pub trait UserRepository {
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError>;
    async fn save(&self, projection: &User) -> Result<(), UnexpectedError>;
}

#[delegate]
pub trait JwtPort {
    fn sign(&self, user: &User) -> Result<Jwt, UnexpectedError>;
    fn verify(&self, token: &Jwt) -> Result<User, UnexpectedError>;
}

pub struct Jwt(pub String);

#[async_trait]
pub trait LinkedInPort {
    async fn sign_in(&self, code: &str) -> Result<LinkedInTokens, UnexpectedError>;
    async fn get_email(&self, tokens: &LinkedInTokens) -> Result<Email, UnexpectedError>;
}

pub struct LinkedInTokens {
    pub access_token: String,
    pub refresh_token: String,
}
