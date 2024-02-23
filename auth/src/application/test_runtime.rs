use super::port::*;
use super::projection::User;
use super::scalar::*;
use super::signal::Signal;
use crate::domain;
use forgen::*;

#[derive(Default)]
pub struct TestRuntime {}

#[async_trait]
impl EventStore for TestRuntime {
    async fn identify_by_email(&self, _email: &Email) -> Result<Option<UserId>, UnexpectedError> {
        Ok(Some(UserId::default()))
    }

    async fn pull_by_user_id(
        &self,
        _user_id: &UserId,
    ) -> Result<Vec<domain::Event>, UnexpectedError> {
        Ok(vec![])
    }

    async fn push(&self, _events: &[domain::Event]) -> Result<(), UnexpectedError> {
        Ok(())
    }
}

#[async_trait]
impl JwtPort for TestRuntime {
    async fn sign(&self, _user: &User) -> Result<Jwt, UnexpectedError> {
        Ok(Jwt("jwt".to_string()))
    }

    async fn verify(&self, _token: &Jwt) -> Result<User, UnexpectedError> {
        Ok(User {
            email: Some(Email::parse("email@example.com").unwrap()),
            user_id: Some(UserId::default()),
        })
    }
}

#[async_trait]
impl SignalPub for TestRuntime {
    async fn publish(&self, _signal: Signal) {}
}

#[async_trait]
impl UserRepository for TestRuntime {
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError> {
        Ok(Some(User {
            email: Some(Email::parse("email@example.com").unwrap()),
            user_id: Some(user_id.clone()),
        }))
    }

    async fn save(&self, _projection: &User) -> Result<(), UnexpectedError> {
        Ok(())
    }
}
