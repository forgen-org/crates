use crate::domain::Event;
use forgen::*;

#[derive(Default)]
pub struct User {
    pub email: String,
}

impl Projection for User {
    type Event = Event;

    fn apply(&mut self, event: &Event) -> &mut Self {
        match event {
            Event::Registered { email, .. } => {
                self.email = email.to_string();
            }
            _ => {}
        }
        self
    }
}
