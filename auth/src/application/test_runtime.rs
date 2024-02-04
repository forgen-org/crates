use crate::application::auth_port::*;
use framework::*;

#[derive(Default)]
pub struct TestRuntime {}

#[async_trait]
impl AuthStore for TestRuntime {
    async fn pull_by_email(&self, _email: &Email) -> Result<Vec<AuthEvent>, AuthStoreError> {
        Ok(vec![])
    }

    async fn pull_by_user_id(&self, _user_id: &UserId) -> Result<Vec<AuthEvent>, AuthStoreError> {
        Ok(vec![])
    }

    async fn push(&self, _events: &[AuthEvent]) -> Result<(), AuthStoreError> {
        Ok(())
    }
}

#[async_trait]
impl UserRepository for TestRuntime {
    async fn find_by_email(&self, email: &Email) -> Result<User, UserRepositoryError> {
        Ok(User {
            email: email.to_string(),
            user_id: UserId::default().to_string(),
        })
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<User, UserRepositoryError> {
        Ok(User {
            email: "email@example.com".to_string(),
            user_id: user_id.to_string(),
        })
    }

    async fn save(&self, _projection: &User) -> Result<(), UserRepositoryError> {
        Ok(())
    }
}

#[async_trait]
impl JwtPort for TestRuntime {
    async fn sign(&self, _user: &User) -> Result<Jwt, JwtPortError> {
        Ok(Jwt("jwt".to_string()))
    }

    async fn verify(&self, _token: &Jwt) -> Result<User, JwtPortError> {
        Ok(User {
            email: "email@example.com".to_string(),
            user_id: UserId::default().to_string(),
        })
    }
}

#[async_trait]
impl Runtime for TestRuntime {}
