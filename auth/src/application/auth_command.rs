use super::auth_port::*;
use crate::domain::{
    auth_message::{AuthError, AuthMessage, RegisterMethod},
    auth_scalar::UserId,
};
use framework::*;

pub struct Register(RegisterMethod);

#[async_trait]
impl<R> Command<R, RegisterError> for Register
where
    R: Runtime + AuthStore + UserRepository,
{
    async fn execute(self, r: &R) -> Result<(), RegisterError> {
        let register_method = self.0;
        let email = match &register_method {
            RegisterMethod::EmailPassword(email, _) => email,
        };

        let existing_users = UserRepository::count_by_email(r, email).await?;

        if existing_users != 0 {
            return Err(RegisterError::UserAlreadyExists);
        }

        let new_events = AuthMessage::Register(register_method).send(&vec![])?;
        let user_id = UserId::new();

        AuthStore::push(r, &user_id, &new_events).await?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum RegisterError {
    #[error(transparent)]
    AuthError(#[from] AuthError),

    #[error(transparent)]
    AuthStoreError(#[from] AuthStoreError),

    #[error("User already exists")]
    UserAlreadyExists,

    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::auth_scalar::Password;

    #[derive(Default)]
    struct MockRuntime {
        existing_email: Option<Email>,
    }

    impl MockRuntime {
        fn existing_email(mut self, existing_email: Email) -> MockRuntime {
            self.existing_email = Some(existing_email);
            self
        }
    }

    #[async_trait]
    impl AuthStore for MockRuntime {
        async fn pull(&self, _user_id: &UserId) -> Result<Vec<AuthEvent>, AuthStoreError> {
            Ok(vec![])
        }
        async fn push(
            &self,
            _user_id: &UserId,
            _events: &[AuthEvent],
        ) -> Result<(), AuthStoreError> {
            Ok(())
        }
    }

    #[async_trait]
    impl UserRepository for MockRuntime {
        async fn count_by_email(&self, email: &Email) -> Result<usize, UserRepositoryError> {
            if let Some(existing_email) = self.existing_email.as_ref() {
                if email.as_str() == existing_email.as_str() {
                    return Ok(1);
                }
            }
            Ok(0)
        }
        async fn find_one_by_credentials(
            &self,
            _credentials: &Credentials,
        ) -> Result<User, UserRepositoryError> {
            unimplemented!()
        }
        async fn save(&self, _projection: &User) -> Result<(), ()> {
            Ok(())
        }
    }

    #[async_trait]
    impl Runtime for MockRuntime {}

    #[tokio::test]
    async fn test_register_user_already_exists() {
        let runtime =
            MockRuntime::default().existing_email(Email::parse("existing@example.com").unwrap());

        let command = Register(RegisterMethod::EmailPassword(
            Email::parse("existing@example.com").unwrap(),
            Password("password".to_string()),
        ));
        assert!(matches!(
            command.execute(&runtime).await,
            Err(RegisterError::UserAlreadyExists)
        ));

        let command = Register(RegisterMethod::EmailPassword(
            Email::parse("nonexisting@example.com").unwrap(),
            Password("password".to_string()),
        ));
        assert!(matches!(command.execute(&runtime).await, Ok(())));
    }
}
