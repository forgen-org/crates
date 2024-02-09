use super::event_bus::EventBus;
use crate::application::port::*;
use crate::domain::scalar::*;
use crate::*;
use framework::*;

pub struct Register {
    pub email: Email,
    pub password: Password,
}

#[async_trait]
impl<R> Execute<R> for Register
where
    R: AuthStore + UserRepository,
    R: Send + Sync,
{
    type Error = CommandError;

    async fn execute(&self, runtime: &R) -> Result<(), CommandError> {
        let existing_events = AuthStore::pull_by_email(runtime, &self.email).await?;

        let new_events = existing_events.dispatch(&domain::Message::Register {
            method: domain::RegisterMethod::EmailPassword {
                email: self.email.clone(),
                password: self.password.clone(),
            },
        })?;

        EventBus(new_events).execute(runtime).await?;

        Ok(())
    }
}

pub struct Login {
    pub email: Email,
    pub password: Password,
}

#[async_trait]
impl<R> Execute<R> for Login
where
    R: AuthStore + UserRepository,
    R: Send + Sync,
{
    type Error = CommandError;

    async fn execute(&self, runtime: &R) -> Result<(), CommandError> {
        let existing_events = AuthStore::pull_by_email(runtime, &self.email).await?;

        let new_events = existing_events.dispatch(&domain::Message::LogIn {
            method: domain::RegisterMethod::EmailPassword {
                email: self.email.clone(),
                password: self.password.clone(),
            },
        })?;

        EventBus(new_events).execute(runtime).await?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error(transparent)]
    DomainError(#[from] domain::Error),
    #[error(transparent)]
    UnexpectedError(#[from] UnexpectedError),
}
