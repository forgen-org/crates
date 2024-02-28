use crate::domain::{
    scalar::{Email, UserId},
    Event,
};
use forgen::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct User {
    pub email: Option<Email>,
    pub user_id: Option<UserId>,
}

impl Project for User {
    type Event = Event;

    fn push(&mut self, event: &Event) -> &mut Self {
        if let Event::Registered { email, user_id } = event {
            self.email = Some(email.clone());
            self.user_id = Some(user_id.clone());
        }
        self
    }
}
