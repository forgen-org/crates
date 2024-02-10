use crate::domain::Event;
use framework::*;

#[derive(Default)]
pub struct User {
    pub email: String,
    pub user_id: String,
}

impl Project<Event> for User {
    fn apply(&mut self, event: &Event) -> &mut Self {
        if let Event::Registered { email, user_id, .. } = event {
            self.email = email.to_string();
            self.user_id = user_id.to_string();
        }
        self
    }
}
