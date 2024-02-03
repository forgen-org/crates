use crate::application::auth_listener::{AuthListener, AuthListenerError};
use crate::application::auth_port::*;
use crate::domain::{
    auth_message::{AuthMessage, RegisterMethod},
    auth_scalar::UserId,
};
use framework::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Register(RegisterMethod);

#[async_trait]
impl<R> Command<R, RegisterError> for Register
where
    R: Runtime + AuthStore + UserRepository,
{
    async fn execute(self, runtime: &R) -> Result<(), RegisterError> {
        let register_method = self.0;
        let email = match &register_method {
            RegisterMethod::EmailPassword(email, _) => email,
        };

        // Check if email already exists
        let existing_users = UserRepository::count_by_email(runtime, email).await?;
        if existing_users != 0 {
            return Err(RegisterError::UserAlreadyExists);
        }

        AuthListener {
            user_id: UserId::new(),
            message: AuthMessage::Register(register_method),
        }
        .execute(runtime)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum RegisterError {
    #[error(transparent)]
    AuthListenerError(#[from] AuthListenerError),
    #[error("User already exists")]
    UserAlreadyExists,
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{application::auth_runtime::AuthRuntime, domain::auth_scalar::Password};

    #[tokio::test]
    async fn test_register_user_already_exists() {
        let runtime =
            AuthRuntime::default().existing_email(Email::parse("existing@example.com").unwrap());

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
