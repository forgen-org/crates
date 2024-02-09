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

    async fn execute(self, runtime: &R) -> Result<(), CommandError> {
        let story = domain::Story(AuthStore::pull_by_email(runtime, &self.email).await?);

        let events = story.dispatch(&domain::Message::Register {
            method: domain::RegisterMethod::EmailPassword {
                email: self.email,
                password: self.password,
            },
        })?;

        EventBus(events).execute(runtime).await?;

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

    async fn execute(self, runtime: &R) -> Result<(), CommandError> {
        let story = domain::Story(AuthStore::pull_by_email(runtime, &self.email).await?);

        let events = story.dispatch(&domain::Message::LogIn {
            method: domain::RegisterMethod::EmailPassword {
                email: self.email,
                password: self.password,
            },
        })?;

        EventBus(events).execute(runtime).await?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error(transparent)]
    DomainError(#[from] domain::Error),
    #[error(transparent)]
    UnexpectedError(#[from] UnexpectedError),
}
