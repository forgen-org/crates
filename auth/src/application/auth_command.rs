use crate::application::auth_port::*;
use crate::domain::auth_message::AuthError;
use crate::domain::auth_message::{AuthMessage, RegisterMethod};
use crate::domain::auth_scalar::{EmailError, Password, PasswordError};
use crate::domain::auth_state::AuthState;
use framework::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterCommand {
    pub email: String,
    pub password: String,
}

#[async_trait]
impl<R> Command<R, AuthCommandError> for RegisterCommand
where
    R: Runtime + AuthStore + UserRepository,
{
    async fn execute(&self, runtime: &R) -> Result<(), AuthCommandError> {
        let email = Email::parse(&self.email)?;
        dispatch(
            runtime,
            &AuthStore::pull_by_email(runtime, &email).await?,
            &AuthMessage::Register(RegisterMethod::EmailPassword(
                email,
                Password::parse(&self.password)?,
            )),
        )
        .await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

#[async_trait]
impl<R> Command<R, AuthCommandError> for LoginCommand
where
    R: Runtime + AuthStore + UserRepository,
{
    async fn execute(&self, runtime: &R) -> Result<(), AuthCommandError> {
        let email = Email::parse(&self.email)?;
        dispatch(
            runtime,
            &AuthStore::pull_by_email(runtime, &email).await?,
            &AuthMessage::LogIn(RegisterMethod::EmailPassword(
                email,
                Password::parse(&self.password)?,
            )),
        )
        .await?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum AuthCommandError {
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    AuthStoreError(#[from] AuthStoreError),
    #[error(transparent)]
    EmailError(#[from] EmailError),
    #[error(transparent)]
    PasswordError(#[from] PasswordError),
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
}

async fn dispatch<R>(
    runtime: &R,
    existing_events: &[AuthEvent],
    message: &AuthMessage,
) -> Result<(), AuthCommandError>
where
    R: Runtime + AuthStore + UserRepository,
{
    let new_events = message.send(existing_events)?;

    // Push new events
    AuthStore::push(runtime, &new_events).await?;

    let state = AuthState(&new_events);

    if let Some(user_id) = state.user_id() {
        // Recompute projections
        let mut projection = UserRepository::find_by_user_id(runtime, user_id).await?;
        projection.apply(&new_events);
        UserRepository::save(runtime, &projection).await?;
    }

    Ok(())
}
