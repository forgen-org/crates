use super::port::*;
use super::scalar::*;
use crate::domain::{Error, Event, Message, State};
use crate::signal::Signal;
use forgen::*;

pub struct Register {
    pub email: Email,
    pub password: Password,
    pub transaction_id: Option<TransactionId>,
}

impl<R> Commander<R> for Register
where
    R: SignalPub + EventStore + UserRepository,
{
    type Error = CommandError;

    fn execute(&self, runtime: &R) -> Result<(), Self::Error> {
        let user_id = EventStore::identify_by_email(runtime, &self.email)?;

        let events = match user_id {
            Some(user_id) => EventStore::pull_by_user_id(runtime, &user_id)?,
            None => vec![],
        };

        let state = State::new(&events);

        let new_events = state.send(&Message::Register {
            email: self.email.clone(),
            password: self.password.clone(),
        })?;

        EventStore::push(runtime, &new_events)?;
        SignalPub::publish(
            runtime,
            Signal::EventsEmitted {
                events: new_events,
                user_id: Some(state.user_id.clone()),
                transaction_id: self.transaction_id.clone(),
            },
        );

        Ok(())
    }
}

pub struct Login {
    pub email: Email,
    pub password: Password,
    pub transaction_id: Option<TransactionId>,
}

impl<R> Commander<R> for Login
where
    R: SignalPub + EventStore,
{
    type Error = CommandError;

    fn execute(&self, runtime: &R) -> Result<(), Self::Error> {
        let user_id = EventStore::identify_by_email(runtime, &self.email)?
            .ok_or(CommandError::EmailNotFound)?;

        let events = EventStore::pull_by_user_id(runtime, &user_id)?;

        let state = State::new(&events);

        let new_events = state.send(&Message::LogIn {
            email: self.email.clone(),
            password: self.password.clone(),
        })?;

        EventStore::push(runtime, &new_events)?;
        SignalPub::publish(
            runtime,
            Signal::EventsEmitted {
                events: new_events,
                user_id: Some(state.user_id.clone()),
                transaction_id: self.transaction_id.clone(),
            },
        );
        Ok(())
    }
}

pub struct ProjectUser {
    pub events: Vec<Event>,
    pub transaction_id: Option<TransactionId>,
    pub user_id: UserId,
}

impl<R> Commander<R> for ProjectUser
where
    R: SignalPub + UserRepository,
{
    type Error = UnexpectedError;

    fn execute(&self, runtime: &R) -> Result<(), Self::Error> {
        let mut user = UserRepository::find_by_user_id(runtime, &self.user_id)?.unwrap_or_default();
        user.extend(&self.events);
        UserRepository::save(runtime, &user)?;
        SignalPub::publish(
            runtime,
            Signal::UserProjected {
                transaction_id: self.transaction_id.clone(),
                user_id: self.user_id.clone(),
            },
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
