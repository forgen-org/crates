use super::{
    error::Error,
    event::Event,
    message::Message,
    scalar::{Email, PasswordHash, UserId},
};
use forgen::*;

#[derive(Default)]
pub struct State {
    pub user_id: UserId,
    pub email: Option<Email>,
    pub password_hash: Option<PasswordHash>,
}

impl Dispatch for State {
    type Error = Error;
    type Event = Event;
    type Message = Message;

    fn dispatch(&self, message: &Self::Message) -> Result<Vec<Self::Event>, Self::Error> {
        match message {
            Message::Register { email, password } => {
                if self.email.is_some() {
                    Err(Error::AlreadyRegistered)
                } else {
                    Ok(vec![
                        Event::Registered {
                            email: email.clone(),
                            user_id: self.user_id.clone(),
                        },
                        Event::PasswordChanged {
                            password_hash: password.into(),
                            user_id: self.user_id.clone(),
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
                    Ok(vec![Event::LoggedIn {
                        user_id: self.user_id.clone(),
                    }])
                } else {
                    Err(Error::InvalidPassword)
                }
            }
            Message::ConnectLinkedIn {
                email,
                access_token,
                refresh_token,
            } => {
                if let Some(existing_email) = &self.email {
                    if existing_email != email {
                        return Err(Error::InvalidEmail);
                    }
                    Ok(vec![Event::LinkedInConnected {
                        access_token: access_token.clone(),
                        refresh_token: refresh_token.clone(),
                        user_id: self.user_id.clone(),
                    }])
                } else {
                    Ok(vec![
                        Event::Registered {
                            email: email.clone(),
                            user_id: self.user_id.clone(),
                        },
                        Event::LinkedInConnected {
                            access_token: access_token.clone(),
                            refresh_token: refresh_token.clone(),
                            user_id: self.user_id.clone(),
                        },
                    ])
                }
            }
        }
    }
}

impl Project for State {
    type Event = Event;

    fn push(&mut self, event: &Self::Event) -> &mut Self {
        match event {
            Event::Registered { email, user_id } => {
                self.email = Some(email.clone());
                self.user_id = user_id.clone();
            }
            Event::PasswordChanged { password_hash, .. } => {
                self.password_hash = Some(password_hash.clone());
            }
            _ => {}
        }
        self
    }
}
