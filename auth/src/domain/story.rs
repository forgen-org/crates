use super::error::Error;
use super::event::{Credentials, Event};
use super::message::{Message, RegisterMethod};
use super::scalar::*;
use framework::*;

pub struct Story(pub Vec<Event>);

impl Dispatch for Story {
    type Event = Event;
    type Message = Message;
    type Error = Error;

    fn dispatch(&self, message: &Self::Message) -> Result<Vec<Self::Event>, Self::Error> {
        match message {
            Message::Register { method } => {
                if self.is_already_registered() {
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

                if let Some((user_id, password_hash)) = self.get_password_hash() {
                    if password_hash.verify(password) {
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

impl Story {
    pub fn is_already_registered(&self) -> bool {
        self.0
            .iter()
            .any(|event| matches!(event, Event::Registered { .. }))
    }

    pub fn get_password_hash(&self) -> Option<(&UserId, &PasswordHash)> {
        self.0.iter().find_map(|event| {
            if let Event::Registered {
                user_id,
                credentials: Credentials::EmailPassword { password_hash, .. },
                ..
            } = event
            {
                Some((user_id, password_hash))
            } else {
                None
            }
        })
    }
}

impl Iterator for Story {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_already_registered() {
        let story = Story(vec![Event::Registered {
            at: Utc.timestamp_opt(0, 0).unwrap(),
            credentials: Credentials::EmailPassword {
                email: Email::parse("email@example.com").unwrap(),
                password_hash: Password::parse("12345678").unwrap().into(),
            },
            user_id: UserId::default(),
        }]);

        let res = story.dispatch(&Message::Register {
            method: RegisterMethod::EmailPassword {
                email: Email::parse("email@example.com").unwrap(),
                password: Password::parse("password").unwrap(),
            },
        });

        assert!(matches!(res, Err(Error::AlreadyRegistered)));
    }
}
