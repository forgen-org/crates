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

        let existing_events = &AuthStore::pull_by_email(runtime, &email).await?;

        let new_events = &AuthMessage::Register {
            method: RegisterMethod::EmailPassword {
                email,
                password: Password::parse(&self.password)?,
            },
        }
        .send(existing_events)?;

        // Push new events
        AuthStore::push(runtime, &new_events).await?;

        // Recompute user projection
        let mut user = User::default();
        user.apply(&new_events);
        UserRepository::save(runtime, &user).await?;

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

        let existing_events = &AuthStore::pull_by_email(runtime, &email).await?;

        let new_events = &AuthMessage::LogIn {
            method: RegisterMethod::EmailPassword {
                email: email.clone(),
                password: Password::parse(&self.password)?,
            },
        }
        .send(existing_events)?;

        // Push new events
        AuthStore::push(runtime, &new_events).await?;

        // Recompute user projection
        let state = AuthState(new_events);
        if let Some(user_id) = state.user_id() {
            recompute_user_projection(runtime, &user_id, new_events).await;
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum AuthCommandError {
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    EmailError(#[from] EmailError),
    #[error(transparent)]
    PasswordError(#[from] PasswordError),
    #[error(transparent)]
    ServiceError(#[from] ServiceError),
}

async fn recompute_user_projection<R>(runtime: &R, user_id: &UserId, new_events: &[AuthEvent])
where
    R: Runtime + UserRepository + AuthStore,
{
    if let Some(mut user) = runtime.find_by_user_id(user_id).await.unwrap() {
        user.apply(&new_events);
        UserRepository::save(runtime, &user).await.unwrap();
    } else {
        let mut user = User::default();
        let existing_events = AuthStore::pull_by_user_id(runtime, user_id).await.unwrap();
        user.apply(&existing_events);
        user.apply(&new_events);
        UserRepository::save(runtime, &user).await.unwrap();
    }
}
