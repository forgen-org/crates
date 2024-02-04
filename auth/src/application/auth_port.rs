pub use crate::application::auth_projection::User;
pub use crate::domain::auth_event::Credentials;
pub use crate::domain::auth_scalar::Email;
pub use crate::domain::{auth_event::AuthEvent, auth_scalar::UserId};
use framework::*;
use serde::{Deserialize, Serialize};

#[async_trait]
#[delegate]
pub trait AuthStore {
    async fn pull_by_email(&self, email: &Email) -> Result<Vec<AuthEvent>, AuthStoreError>;
    async fn pull_by_user_id(&self, user_id: &UserId) -> Result<Vec<AuthEvent>, AuthStoreError>;
    async fn push(&self, events: &[AuthEvent]) -> Result<(), AuthStoreError>;
}

#[derive(Debug, Error)]
pub enum AuthStoreError {
    #[error("Database error")]
    DatabaseError,
}

#[async_trait]
#[delegate]
pub trait UserRepository {
    async fn find_by_email(&self, email: &Email) -> Result<User, UserRepositoryError>;
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<User, UserRepositoryError>;
    async fn save(&self, projection: &User) -> Result<(), UserRepositoryError>;
}

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("User not found")]
    UserNotFound,
}

#[async_trait]
#[delegate]
pub trait JwtPort {
    async fn sign(&self, user: &User) -> Result<Jwt, JwtPortError>;
    async fn verify(&self, token: &Jwt) -> Result<User, JwtPortError>;
}

#[derive(Debug, Error)]
pub enum JwtPortError {
    #[error("Unknown error")]
    UnknownError,
}

#[derive(Serialize, Deserialize)]
pub struct Jwt(pub String);
