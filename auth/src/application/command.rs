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

#[async_trait]
impl<R> Commander<R> for Register
where
    R: EventStore + SignalPub + UserRepository,
    R: Send + Sync,
{
    type Error = CommandError;

    async fn execute(&self, runtime: &R) -> Result<(), Self::Error> {
        let user_id = EventStore::identify_by_email(runtime, &self.email).await?;

        let events = match user_id {
            Some(user_id) => EventStore::pull_by_user_id(runtime, &user_id).await?,
            None => vec![],
        };

        let state = State::new(&events);

        let new_events = state.send(&Message::Register {
            email: self.email.clone(),
            password: self.password.clone(),
        })?;

        EventStore::push(runtime, &new_events).await?;
        SignalPub::publish(
            runtime,
            Signal::EventsEmitted {
                events: new_events,
                user_id: Some(state.user_id.clone()),
                transaction_id: self.transaction_id.clone(),
            },
        )
        .await;

        Ok(())
    }
}

pub struct Login {
    pub email: Email,
    pub password: Password,
    pub transaction_id: Option<TransactionId>,
}

#[async_trait]
impl<R> Commander<R> for Login
where
    R: EventStore + SignalPub,
    R: Send + Sync,
{
    type Error = CommandError;

    async fn execute(&self, runtime: &R) -> Result<(), Self::Error> {
        let user_id = EventStore::identify_by_email(runtime, &self.email)
            .await?
            .ok_or(CommandError::EmailNotFound)?;

        let events = EventStore::pull_by_user_id(runtime, &user_id).await?;

        let state = State::new(&events);

        let new_events = state.send(&Message::LogIn {
            email: self.email.clone(),
            password: self.password.clone(),
        })?;

        EventStore::push(runtime, &new_events).await?;
        SignalPub::publish(
            runtime,
            Signal::EventsEmitted {
                events: new_events,
                user_id: Some(state.user_id.clone()),
                transaction_id: self.transaction_id.clone(),
            },
        )
        .await;
        Ok(())
    }
}

#[cfg(feature = "linkedin")]
pub struct ConnectLinkedIn {
    pub code: String,
    pub transaction_id: Option<TransactionId>,
}

#[cfg(feature = "linkedin")]
#[async_trait]
impl<R> Commander<R> for ConnectLinkedIn
where
    R: EventStore + LinkedInPort + SignalPub,
    R: Send + Sync,
{
    type Error = CommandError;

    async fn execute(&self, runtime: &R) -> Result<(), Self::Error> {
        let tokens = LinkedInPort::sign_in(runtime, &self.code).await?;

        let email = LinkedInPort::get_email(runtime, &tokens).await?;

        let user_id = EventStore::identify_by_email(runtime, &email).await?;

        let events = match user_id {
            Some(user_id) => EventStore::pull_by_user_id(runtime, &user_id).await?,
            None => vec![],
        };

        let state = State::new(&events);

        let new_events = state.send(&Message::ConnectLinkedIn {
            email: email.clone(),
            access_token: tokens.access_token.clone(),
            refresh_token: tokens.refresh_token.clone(),
        })?;

        EventStore::push(runtime, &new_events).await?;
        SignalPub::publish(
            runtime,
            Signal::EventsEmitted {
                events: new_events,
                user_id: Some(state.user_id.clone()),
                transaction_id: self.transaction_id.clone(),
            },
        )
        .await;
        Ok(())
    }
}

pub struct ProjectUser {
    pub events: Vec<Event>,
    pub transaction_id: Option<TransactionId>,
    pub user_id: UserId,
}

#[async_trait]
impl<R> Commander<R> for ProjectUser
where
    R: SignalPub + UserRepository,
    R: Send + Sync,
{
    type Error = UnexpectedError;

    async fn execute(&self, runtime: &R) -> Result<(), Self::Error> {
        let mut user = UserRepository::find_by_user_id(runtime, &self.user_id)
            .await?
            .unwrap_or_default();
        user.extend(&self.events);
        UserRepository::save(runtime, &user).await?;
        SignalPub::publish(
            runtime,
            Signal::UserProjected {
                transaction_id: self.transaction_id.clone(),
                user_id: self.user_id.clone(),
            },
        )
        .await;
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
