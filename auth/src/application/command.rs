use super::port::*;
use super::scalar::*;
use crate::domain::{Auth, Error, Message};
use crate::signal::{Metadata, Signal};
use forgen::*;

pub struct Register {
    pub email: Email,
    pub password: Password,
    pub transaction_id: Option<TransactionId>,
}

impl<R> Command<R> for Register
where
    R: SignalBus + EventStore + UserRepository,
{
    type Error = CommandError;

    fn execute(&self, runtime: &R) -> Result<(), Self::Error> {
        let user_id = EventStore::identify_by_email(runtime, &self.email)?;

        let events = match user_id {
            Some(user_id) => EventStore::pull_by_user_id(runtime, &user_id)?,
            None => vec![],
        };

        let state = Auth::new(&events);

        let new_events = state.send(&Message::Register {
            email: self.email.clone(),
            password: self.password.clone(),
        })?;

        let user_id = UserId::default();

        EventStore::push(runtime, &user_id, &new_events)?;
        SignalBus::publish(
            runtime,
            Signal::EventsEmitted(new_events, Metadata::new().with_user_id(user_id)),
        );

        Ok(())
    }
}

pub struct Login {
    pub email: Email,
    pub password: Password,
    pub transaction_id: Option<TransactionId>,
}

impl<R> Command<R> for Login
where
    R: SignalBus + EventStore,
{
    type Error = CommandError;

    fn execute(&self, runtime: &R) -> Result<(), Self::Error> {
        let user_id = EventStore::identify_by_email(runtime, &self.email)?
            .ok_or(CommandError::EmailNotFound)?;

        let events = EventStore::pull_by_user_id(runtime, &user_id)?;

        let state = Auth::new(&events);

        let new_events = state.send(&Message::LogIn {
            email: self.email.clone(),
            password: self.password.clone(),
        })?;

        EventStore::push(runtime, &user_id, &new_events)?;
        SignalBus::publish(
            runtime,
            Signal::EventsEmitted(new_events, Metadata::new().with_user_id(user_id)),
        );
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error(transparent)]
    DomainError(#[from] Error),
    #[error("Email not found")]
    EmailNotFound,
    #[error(transparent)]
    UnexpectedError(#[from] UnexpectedError),
}
