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

impl Projector for User {
    type Event = Event;

    fn push(&mut self, event: &Event) -> &mut Self {
        if let Event::Registered { email, user_id } = event {
            self.email = Some(email.clone());
            self.user_id = Some(user_id.clone());
        }
        self
    }
}
