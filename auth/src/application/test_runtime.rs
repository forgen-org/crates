use super::event::Event;
use super::port::*;
use super::projection::User;
use super::scalar::*;
use framework::*;

#[derive(Default)]
pub struct TestRuntime {}

#[async_trait]
impl EventStore for TestRuntime {
    async fn identify_by_email(&self, _email: &Email) -> Result<Option<UserId>, UnexpectedError> {
        Ok(Some(UserId::default()))
    }

    async fn pull_by_user_id(&self, _user_id: &UserId) -> Result<Vec<Event>, UnexpectedError> {
        Ok(vec![])
    }

    async fn push(&self, _events: &[Event]) -> Result<(), UnexpectedError> {
        Ok(())
    }
}

#[async_trait]
impl UserRepository for TestRuntime {
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
