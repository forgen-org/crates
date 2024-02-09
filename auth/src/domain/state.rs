use framework::Project;

use super::event::{Credentials, Event};
use super::scalar::*;

#[derive(Default)]
pub struct State {
    pub user_id: Option<UserId>,
    pub is_already_registered: bool,
    pub password_hash: Option<PasswordHash>,
}

impl State {
    pub fn verify(&self, password: &Password) -> bool {
        if let Some(password_hash) = &self.password_hash {
            password_hash.verify(password)
        } else {
            false
        }
    }
}

impl Project<Event> for State {
    fn apply(&mut self, event: &Event) -> &mut Self {
        match event {
            Event::Registered {
                user_id,
                credentials,
                ..
            } => {
                self.user_id = Some(user_id.clone());
                self.is_already_registered = true;
                self.password_hash = match credentials {
                    Credentials::EmailPassword { password_hash, .. } => Some(password_hash.clone()),
                };
            }
            _ => {}
        }
        self
    }
}
