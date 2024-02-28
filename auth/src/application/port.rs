use super::projection::User;
use crate::domain::{scalar::*, Event};
use forgen::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[async_trait]
#[delegate]
pub trait EventStore {
    async fn identify_by_email(&self, email: &Email) -> Result<Option<UserId>, UnexpectedError>;
    async fn pull_by_user_id(&self, user_id: &UserId) -> Result<Vec<Event>, UnexpectedError>;
    async fn push(&self, events: &[Event]) -> Result<(), UnexpectedError>;
}

#[derive(Serialize)]
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
    pub refresh_token: Option<String>,
}

#[delegate]
pub trait Observer<T> {
    fn notify(&self, value: &T);
}

#[derive(Clone)]
pub enum Signal {
    EventsEmitted {
        events: Vec<Event>,
        transaction_id: Option<TransactionId>,
        user_id: Option<UserId>,
    },
    UserProjected {
        transaction_id: Option<TransactionId>,
        user_id: UserId,
    },
}

#[async_trait]
#[delegate]
pub trait SignalPub {
    async fn publish(&self, signal: Signal);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionId(pub Uuid);

impl Default for TransactionId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl ToString for TransactionId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[async_trait]
#[delegate]
pub trait UserRepository {
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError>;
    async fn save(&self, projection: &User) -> Result<(), UnexpectedError>;
}

#[async_trait]
#[delegate]
pub trait WebView {
    fn get_query_param(&self, key: &str) -> Option<String>;
    async fn push(&self, url: &str) -> Result<(), UnexpectedError>;
}
