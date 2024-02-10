use crate::application::{
    event::Event,
    scalar::{Email, PasswordHash, UserId},
};
use chrono::{DateTime, Utc};
use framework::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "_tag")]
pub enum EventDto {
    Registered {
        at: DateTime<Utc>,
        email: String,
        user_id: String,
    },
    PasswordChanged {
        at: DateTime<Utc>,
        password_hash: [u8; 32],
        user_id: String,
    },
    #[cfg(feature = "linkedin")]
    LinkedInConnected {
        access_token: String,
        at: DateTime<Utc>,
        refresh_token: String,
        user_id: String,
    },
    LoggedIn {
        at: DateTime<Utc>,
        user_id: String,
    },
}

impl From<&Event> for EventDto {
    fn from(event: &Event) -> Self {
        match event {
            Event::Registered { at, email, user_id } => EventDto::Registered {
                at: at.clone(),
                email: email.to_string(),
                user_id: user_id.to_string(),
            },
            Event::PasswordChanged {
                at,
                password_hash,
                user_id,
            } => EventDto::PasswordChanged {
                at: at.clone(),
                password_hash: password_hash.0,
                user_id: user_id.to_string(),
            },
            #[cfg(feature = "linkedin")]
            Event::LinkedInConnected {
                access_token,
                at,
                refresh_token,
                user_id,
            } => EventDto::LinkedInConnected {
                access_token: access_token.clone(),
                at: at.clone(),
                refresh_token: refresh_token.clone(),
                user_id: user_id.to_string(),
            },
            Event::LoggedIn { at, user_id } => EventDto::LoggedIn {
                at: at.clone(),
                user_id: user_id.to_string(),
            },
        }
    }
}

impl TryFrom<EventDto> for Event {
    type Error = UnexpectedError;

    fn try_from(dto: EventDto) -> Result<Event, Self::Error> {
        Ok(match dto {
            EventDto::Registered { at, email, user_id } => Event::Registered {
                at,
                email: Email::parse(email).map_err(UnexpectedError::from)?,
                user_id: UserId::parse(&user_id).map_err(UnexpectedError::from)?,
            },
            EventDto::PasswordChanged {
                at,
                password_hash,
                user_id,
            } => Event::PasswordChanged {
                at,
                password_hash: PasswordHash(password_hash),
                user_id: UserId::parse(&user_id).map_err(UnexpectedError::from)?,
            },
            #[cfg(feature = "linkedin")]
            EventDto::LinkedInConnected {
                access_token,
                at,
                refresh_token,
                user_id,
            } => Event::LinkedInConnected {
                access_token,
                at,
                refresh_token,
                user_id: UserId::parse(&user_id).map_err(UnexpectedError::from)?,
            },
            EventDto::LoggedIn { at, user_id } => Event::LoggedIn {
                at,
                user_id: UserId::parse(&user_id).map_err(UnexpectedError::from)?,
            },
        })
    }
}
