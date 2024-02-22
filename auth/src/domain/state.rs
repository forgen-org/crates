use super::error::Error;
use super::event::Event;
use super::message::Message;
use super::scalar::{Email, PasswordHash};
use forgen::State;

#[derive(Default)]
pub struct Auth {
    pub email: Option<Email>,
    pub password_hash: Option<PasswordHash>,
}

impl State for Auth {
    type Error = Error;
    type Event = Event;
    type Message = Message;

    fn send(&self, message: &Self::Message) -> Result<Vec<Self::Event>, Self::Error> {
        match message {
            Message::Register { email, password } => {
                if self.email.is_some() {
                    Err(Error::AlreadyRegistered)
                } else {
                    Ok(vec![
                        Event::Registered {
                            email: email.clone(),
                        },
                        Event::PasswordChanged {
                            password_hash: password.into(),
                        },
                    ])
                }
            }
            Message::LogIn { password, .. } => {
                if self
                    .password_hash
                    .as_ref()
                    .map(|hash| hash.verify(password))
                    .unwrap_or(false)
                {
                    Ok(vec![Event::LoggedIn])
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
                if let Some(existing_email) = &self.email {
                    if existing_email != email {
                        return Err(Error::InvalidEmail);
                    }
                    Ok(vec![
                        Event::Registered {
                            email: email.clone(),
                        },
                        Event::LinkedInConnected {
                            access_token: access_token.clone(),
                            refresh_token: refresh_token.clone(),
                        },
                    ])
                } else {
                    Ok(vec![Event::LinkedInConnected {
                        access_token: access_token.clone(),
                        refresh_token: refresh_token.clone(),
                    }])
                }
            }
        }
    }

    fn apply(&mut self, event: &Event) -> &mut Self {
        match event {
            Event::Registered { email } => {
                self.email = Some(email.clone());
            }
            Event::PasswordChanged { password_hash, .. } => {
                self.password_hash = Some(password_hash.clone());
            }
            _ => {}
        }
        self
    }
}
