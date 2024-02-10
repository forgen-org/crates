// use super::event_bus::EventBus;
use crate::application::port::*;
use crate::domain;
use crate::domain::scalar::*;
use framework::*;

pub struct Register {
    pub email: Email,
    pub password: Password,
}

#[async_trait]
impl<R> Execute<R> for Register
where
    R: EventBus + EventStore + UserRepository,
    R: Send + Sync,
{
    type Error = CommandError;

    async fn execute(&self, runtime: &R) -> Result<(), CommandError> {
        let user_id = EventStore::identify_by_email(runtime, &self.email).await?;

        let existing_events = match user_id {
            Some(user_id) => EventStore::pull_by_user_id(runtime, &user_id).await?,
            None => vec![],
        };

        let new_events = existing_events.dispatch(&domain::Message::Register {
            email: self.email.clone(),
            password: self.password.clone(),
        })?;

        EventStore::push(runtime, &new_events).await?;
        EventBus::publish(runtime, new_events);

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
    R: EventBus + EventStore,
    R: Send + Sync,
{
    type Error = CommandError;

    async fn execute(&self, runtime: &R) -> Result<(), CommandError> {
        let user_id = EventStore::identify_by_email(runtime, &self.email).await?;

        let existing_events = match user_id {
            Some(user_id) => EventStore::pull_by_user_id(runtime, &user_id).await?,
            None => vec![],
        };

        let new_events = existing_events.dispatch(&domain::Message::LogIn {
            email: self.email.clone(),
            password: self.password.clone(),
        })?;

        EventStore::push(runtime, &new_events).await?;
        EventBus::publish(runtime, new_events);

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
