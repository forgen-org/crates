use super::projection::User;
use super::transaction::Transaction;
use crate::domain;
use crate::domain::scalar::*;
use crate::domain::Event;
use framework::*;

#[async_trait]
#[delegate]
pub trait EventBus {
    fn publish(&self, events: Vec<Event>) -> TransactionId;
    fn subscribe(&self) -> EventStream<Event>;
}

#[async_trait]
#[delegate]
pub trait TransactionBus {
    fn publish(&self, transaction: Transaction);
    fn subscribe(&self) -> TransactionStream<Transaction>;
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
