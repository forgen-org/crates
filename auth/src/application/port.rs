use super::projection::User;
use crate::domain::scalar::*;
use crate::domain::Event;
use framework::*;
use serde::{Deserialize, Serialize};

#[async_trait]
#[delegate]
pub trait AuthStore {
    async fn pull_by_email(&self, email: &Email) -> Result<Vec<Event>, UnexpectedError>;
    async fn pull_by_user_id(&self, user_id: &UserId) -> Result<Vec<Event>, UnexpectedError>;
    async fn push(&self, events: &[Event]) -> Result<(), UnexpectedError>;
}

#[async_trait]
#[delegate]
pub trait UserRepository {
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UnexpectedError>;
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError>;
    async fn save(&self, projection: &User) -> Result<(), UnexpectedError>;
}

#[delegate]
pub trait JwtPort {
    fn sign(&self, user: &User) -> Result<Jwt, UnexpectedError>;
    fn verify(&self, token: &Jwt) -> Result<User, UnexpectedError>;
}

#[derive(Serialize, Deserialize)]
pub struct Jwt(pub String);
