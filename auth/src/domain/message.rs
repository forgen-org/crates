use super::error::Error;
use super::event::{Event, State};
use super::scalar::*;
use chrono::Utc;
use framework::*;

pub enum Message {
    Register {
        email: Email,
        password: Password,
    },
    LogIn {
        email: Email,
        password: Password,
    },
    #[cfg(feature = "linkedin")]
    ConnectLinkedIn {
        email: Email,
        access_token: String,
        refresh_token: String,
    },
}

impl Dispatch<Event> for Message {
    type Error = Error;

    fn dispatch(&self, events: &[Event]) -> Result<Vec<Event>, Self::Error> {
        match self {
            Message::Register { email, password } => {
                if events.is_already_registered().is_some() {
                    Err(Error::AlreadyRegistered)
                } else {
                    let user_id = UserId::default();
                    Ok(vec![
                        Event::Registered {
                            at: Utc::now(),
                            email: email.clone(),
                            user_id: user_id.clone(),
                        },
                        Event::PasswordChanged {
                            at: Utc::now(),
                            password_hash: password.into(),
                            user_id: user_id.clone(),
                        },
                    ])
                }
            }
            Message::LogIn { email, password } => {
                if let Some(user_id) = events.is_password_valid(password) {
                    Ok(vec![Event::LoggedIn {
                        at: Utc::now(),
                        user_id: user_id.clone(),
                    }])
                } else {
                    Err(Error::InvalidPassword)
                }
            }
            #[cfg(feature = "linkedin")]
            Message::ConnectLinkedIn {
                email,
                access_token,
                refresh_token,
            } => {
                if let Some(user_id) = events.is_already_registered() {
                    Ok(vec![Event::LinkedInConnected {
                        access_token: access_token.clone(),
                        at: Utc::now(),
                        refresh_token: refresh_token.clone(),
                        user_id: user_id.clone(),
                    }])
                } else {
                    let user_id = UserId::default();
                    Ok(vec![
                        Event::Registered {
                            at: Utc::now(),
                            email: email.clone(),
                            user_id: user_id.clone(),
                        },
                        Event::LinkedInConnected {
                            access_token: access_token.clone(),
                            at: Utc::now(),
                            refresh_token: refresh_token.clone(),
                            user_id: user_id.clone(),
                        },
                    ])
                }
            }
        }
    }
}
