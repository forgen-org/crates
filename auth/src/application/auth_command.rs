use crate::application::auth_listener::{AuthListener, AuthListenerError};
use crate::application::auth_port::*;
use crate::domain::auth_scalar::{EmailError, Password, PasswordError};
use crate::domain::{
    auth_message::{AuthMessage, RegisterMethod},
    auth_scalar::UserId,
};
use framework::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Register {
    email: String,
    password: String,
}

#[async_trait]
impl<R> Command<R, RegisterError> for Register
where
    R: Runtime + AuthStore + UserRepository,
{
    async fn execute(self, runtime: &R) -> Result<(), RegisterError> {
        let email = Email::parse(self.email)?;

        // Check if email already exists
        let existing_users = UserRepository::count_by_email(runtime, &email).await?;
        if existing_users != 0 {
            return Err(RegisterError::UserAlreadyExists);
        }

        let register_method = RegisterMethod::EmailPassword(email, Password::parse(self.password)?);
        AuthListener {
            user_id: UserId::default(),
            message: AuthMessage::Register(register_method),
        }
        .execute(runtime)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum RegisterError {
    #[error("User already exists")]
    UserAlreadyExists,

    #[error(transparent)]
    AuthListenerError(#[from] AuthListenerError),
    #[error(transparent)]
    EmailError(#[from] EmailError),
    #[error(transparent)]
    PasswordError(#[from] PasswordError),
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::test_runtime::TestRuntime;

    #[tokio::test]
    async fn test_register_user_already_exists() {
        let runtime =
            TestRuntime::default().existing_email(Email::parse("existing@example.com").unwrap());

        let command = Register {
            email: "existing@example.com".to_string(),
            password: "password".to_string(),
        };
        assert!(matches!(
            command.execute(&runtime).await,
            Err(RegisterError::UserAlreadyExists)
        ));

        let command = Register {
            email: "nonexisting@example.com".to_string(),
            password: "password".to_string(),
        };
        assert!(matches!(command.execute(&runtime).await, Ok(())));
    }
}
