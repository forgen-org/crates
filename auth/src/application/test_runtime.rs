use super::projection::User;
use crate::*;
use framework::*;

#[derive(Default)]
pub struct TestRuntime {}

#[async_trait]
impl AuthStore for TestRuntime {
    async fn pull_by_email(&self, _email: &Email) -> Result<Vec<domain::Event>, UnexpectedError> {
        Ok(vec![])
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
impl UserRepository for TestRuntime {
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UnexpectedError> {
        Ok(Some(User {
            email: email.to_string(),
            user_id: UserId::default().to_string(),
        }))
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError> {
        Ok(Some(User {
            email: "email@example.com".to_string(),
            user_id: user_id.to_string(),
        }))
    }

    async fn save(&self, _projection: &User) -> Result<(), UnexpectedError> {
        Ok(())
    }
}
impl JwtPort for TestRuntime {
    fn sign(&self, _user: &User) -> Result<Jwt, UnexpectedError> {
        Ok(Jwt("jwt".to_string()))
    }

    fn verify(&self, _token: &Jwt) -> Result<User, UnexpectedError> {
        Ok(User {
            email: "email@example.com".to_string(),
            user_id: UserId::default().to_string(),
        })
    }
}

impl Framework for TestRuntime {}
