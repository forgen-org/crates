use super::projection::User;
use super::view::{Jwt, LinkedInOAuthUrl};
use crate::domain::{scalar::*, Event};
use forgen::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[port]
pub trait EventStore {
    async fn identify_by_email(&self, email: &Email) -> Result<Option<UserId>, UnexpectedError>;
    async fn pull_by_user_id(&self, user_id: &UserId) -> Result<Vec<Event>, UnexpectedError>;
    async fn push(&self, events: &[Event]) -> Result<(), UnexpectedError>;
}

#[port]
pub trait JwtPort {
    async fn sign(&self, user: &User) -> Result<Jwt, UnexpectedError>;
    async fn verify(&self, token: &Jwt) -> Result<User, UnexpectedError>;
}

#[port]
pub trait JwtStore {
    async fn get(&self) -> Option<Jwt>;
    async fn set(&self, jwt: &Jwt);
}

#[port]
pub trait LinkedInApi {
    async fn get_email(&self, tokens: &LinkedInTokens) -> Result<Email, UnexpectedError>;
    async fn get_oauth_url(&self) -> Result<LinkedInOAuthUrl, UnexpectedError>;
    async fn sign_in(&self, code: &str) -> Result<LinkedInTokens, UnexpectedError>;
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

#[port]
pub trait SignalPub {
    async fn publish(&self, signal: Signal);
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
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

#[port]
pub trait UserRepository {
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError>;
    async fn save(&self, projection: &User) -> Result<(), UnexpectedError>;
}
