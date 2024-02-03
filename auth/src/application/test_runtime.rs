use crate::{application::auth_port::*, domain::auth_scalar::Password};
use framework::*;

#[derive(Default)]
pub struct TestRuntime {
    existing_email: Option<Email>,
}

impl TestRuntime {
    pub fn existing_email(mut self, existing_email: Email) -> TestRuntime {
        self.existing_email = Some(existing_email);
        self
    }
}

#[async_trait]
impl AuthStore for TestRuntime {
    async fn pull(&self, _user_id: &UserId) -> Result<Vec<AuthEvent>, AuthStoreError> {
        Ok(vec![])
    }

    async fn push(&self, _user_id: &UserId, _events: &[AuthEvent]) -> Result<(), AuthStoreError> {
        Ok(())
    }
}

#[async_trait]
impl UserRepository for TestRuntime {
    async fn count_by_email(&self, email: &Email) -> Result<usize, UserRepositoryError> {
        if let Some(existing_email) = self.existing_email.as_ref() {
            if email.as_str() == existing_email.as_str() {
                return Ok(1);
            }
        }
        Ok(0)
    }

    async fn find_one(&self, _user_id: &UserId) -> Result<User, UserRepositoryError> {
        Ok(User {
            password_hash: Some(Password("password_hash".to_string()).into()),
        })
    }

    async fn find_one_by_credentials(
        &self,
        _credentials: &Credentials,
    ) -> Result<User, UserRepositoryError> {
        unimplemented!()
    }

    async fn save(&self, _user_id: &UserId, _projection: &User) -> Result<(), UserRepositoryError> {
        Ok(())
    }
}

#[async_trait]
impl Runtime for TestRuntime {}
