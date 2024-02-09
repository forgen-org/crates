use super::error::Error;
use super::event::Event;
use super::state::State;
use super::{event::Credentials, scalar::*};
use framework::*;

pub enum Message {
    Register { method: RegisterMethod },
    LogIn { method: RegisterMethod },
}

pub enum RegisterMethod {
    EmailPassword { email: Email, password: Password },
}

impl From<&RegisterMethod> for Credentials {
    fn from(method: &RegisterMethod) -> Self {
        match method {
            RegisterMethod::EmailPassword { email, password } => Credentials::EmailPassword {
                email: email.clone(),
                password_hash: password.into(),
            },
        }
    }
}

impl Dispatch<Event> for Message {
    type Error = Error;

    fn dispatch(&self, events: &[Event]) -> Result<Vec<Event>, Self::Error> {
        let state = State::project(events);

        match self {
            Message::Register { method } => {
                if state.is_already_registered {
                    Err(Error::AlreadyRegistered)
                } else {
                    let user_id = UserId::default();
                    Ok(vec![Event::Registered {
                        at: chrono::Utc::now(),
                        user_id,
                        credentials: method.into(),
                    }])
                }
            }
            Message::LogIn { method } => {
                let RegisterMethod::EmailPassword { password, .. } = method;

                if let Some(user_id) = &state.user_id {
                    if state.verify(password) {
                        Ok(vec![Event::LoggedIn {
                            at: chrono::Utc::now(),
                            user_id: user_id.clone(),
                        }])
                    } else {
                        Err(Error::InvalidPassword)
                    }
                } else {
                    Err(Error::NotRegistered)
                }
            }
        }
    }
}
