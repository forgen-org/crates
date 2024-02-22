use crate::{
    domain::Event,
    scalar::{Email, UserId},
};
use forgen::*;

#[derive(Default)]
pub struct User {
    pub email: Option<Email>,
    pub user_id: Option<UserId>,
}

impl Projection for User {
    type Event = Event;

    fn apply(&mut self, event: &Event) -> &mut Self {
        match event {
            Event::Registered { email, user_id } => {
                self.email = Some(email.clone());
                self.user_id = Some(user_id.clone());
            }
            _ => {}
        }
        self
    }
}
