use framework::Projection;
use serde::{Deserialize, Serialize};

use crate::domain::auth_event::AuthEvent;

use super::auth_port::Credentials;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub user_id: String,
}

impl Projection for User {
    type Event = AuthEvent;

    fn apply(&mut self, events: &[Self::Event]) {
        for event in events {
            if let AuthEvent::Registered(user_id, credentials) = event {
                self.user_id = user_id.to_string();
                self.email = match credentials {
                    Credentials::EmailPassword(email, _) => email.to_string(),
                };
            }
        }
    }
}
