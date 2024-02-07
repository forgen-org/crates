pub use crate::application::auth_projection::User;
pub use crate::domain::auth_event::Credentials;
pub use crate::domain::auth_scalar::Email;
pub use crate::domain::{auth_event::AuthEvent, auth_scalar::UserId};
use framework::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Service error: {0}")]
    UnknownError(String),
}

impl From<String> for ServiceError {
    fn from(error: String) -> Self {
        ServiceError::UnknownError(error)
    }
}

#[async_trait]
#[delegate]
pub trait AuthStore {
    async fn pull_by_email(&self, email: &Email) -> Result<Vec<AuthEvent>, ServiceError>;
    async fn pull_by_user_id(&self, user_id: &UserId) -> Result<Vec<AuthEvent>, ServiceError>;
    async fn push(&self, events: &[AuthEvent]) -> Result<(), ServiceError>;
}

#[async_trait]
#[delegate]
pub trait UserRepository {
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, ServiceError>;
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, ServiceError>;
    async fn save(&self, projection: &User) -> Result<(), ServiceError>;
}

#[delegate]
pub trait JwtPort {
    fn sign(&self, user: &User) -> Result<Jwt, ServiceError>;
    fn verify(&self, token: &Jwt) -> Result<User, ServiceError>;
}

#[derive(Serialize, Deserialize)]
pub struct Jwt(pub String);
