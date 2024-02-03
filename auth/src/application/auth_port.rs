pub use crate::application::auth_projection::User;
pub use crate::domain::auth_event::Credentials;
pub use crate::domain::auth_scalar::Email;
pub use crate::domain::{auth_event::AuthEvent, auth_scalar::UserId};
use framework::*;

#[async_trait]
#[delegate]
pub trait AuthStore {
    async fn pull(&self, user_id: &UserId) -> Result<Vec<AuthEvent>, AuthStoreError>;
    async fn push(&self, user_id: &UserId, events: &[AuthEvent]) -> Result<(), AuthStoreError>;
}

#[derive(Debug, Error)]
pub enum AuthStoreError {}

#[async_trait]
#[delegate]
pub trait UserRepository {
    async fn count_by_email(&self, email: &Email) -> Result<usize, UserRepositoryError>;
    async fn find_one(&self, user_id: &UserId) -> Result<User, UserRepositoryError>;
    async fn find_one_by_credentials(
        &self,
        credentials: &Credentials,
    ) -> Result<User, UserRepositoryError>;
    async fn save(&self, user_id: &UserId, projection: &User) -> Result<(), UserRepositoryError>;
}

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("User not found")]
    UserNotFound,
}

#[async_trait]
#[delegate]
pub trait JwtService {
    async fn sign(&self, user: &User) -> Result<Jwt, ()>;
    async fn verify(&self, token: &Jwt) -> Result<User, ()>;
}

pub type Jwt = jwt::Token<jwt::Header, User, hmac::Hmac<sha2::Sha256>>;
