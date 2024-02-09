use crate::domain::{Credentials, Event};
use framework::*;

#[derive(Default)]
pub struct User {
    pub email: String,
    pub user_id: String,
}

impl Project for User {
    type Event = Event;

    fn apply(&mut self, event: &Self::Event) -> &mut Self {
        if let Event::Registered {
            user_id,
            credentials,
            ..
        } = event
        {
            self.user_id = user_id.to_string();
            self.email = match credentials {
                Credentials::EmailPassword { email, .. } => email.to_string(),
            };
        }
        self
    }
}
