use crate::application::{
    scalar::{Email, PasswordHash, UserId},
    Event,
};
use chrono::{DateTime, Utc};
use forgen::*;
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
            Event::Registered { email, user_id } => EventDto::Registered {
                at: Utc::now(),
                email: email.to_string(),
                user_id: user_id.to_string(),
            },
            Event::PasswordChanged {
                password_hash,
                user_id,
            } => EventDto::PasswordChanged {
                at: Utc::now(),
                password_hash: password_hash.0,
                user_id: user_id.to_string(),
            },
            #[cfg(feature = "linkedin")]
            Event::LinkedInConnected {
                access_token,

                refresh_token,
                user_id,
            } => EventDto::LinkedInConnected {
                access_token: access_token.clone(),
                at: Utc::now(),
                refresh_token: refresh_token.clone(),
                user_id: user_id.to_string(),
            },
            Event::LoggedIn { user_id } => EventDto::LoggedIn {
                at: Utc::now(),
                user_id: user_id.to_string(),
            },
        }
    }
}

impl TryFrom<EventDto> for Event {
    type Error = UnexpectedError;

    fn try_from(dto: EventDto) -> Result<Event, Self::Error> {
        Ok(match dto {
            EventDto::Registered { email, user_id, .. } => Event::Registered {
                email: Email::parse(email).map_err(UnexpectedError::from)?,
                user_id: UserId::parse(&user_id).map_err(UnexpectedError::from)?,
            },
            EventDto::PasswordChanged {
                password_hash,
                user_id,
                ..
            } => Event::PasswordChanged {
                password_hash: PasswordHash(password_hash),
                user_id: UserId::parse(&user_id).map_err(UnexpectedError::from)?,
            },
            #[cfg(feature = "linkedin")]
            EventDto::LinkedInConnected {
                access_token,
                refresh_token,
                user_id,
                ..
            } => Event::LinkedInConnected {
                access_token,
                refresh_token,
                user_id: UserId::parse(&user_id).map_err(UnexpectedError::from)?,
            },
            EventDto::LoggedIn { user_id, .. } => Event::LoggedIn {
                user_id: UserId::parse(&user_id).map_err(UnexpectedError::from)?,
            },
        })
    }
}
