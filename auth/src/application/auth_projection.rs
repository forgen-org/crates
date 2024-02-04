use framework::Projection;
use serde::{Deserialize, Serialize};

use crate::domain::{auth_event::AuthEvent, auth_scalar::PasswordHash};

#[derive(Default, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing)]
    pub password_hash: Option<PasswordHash>,
}

impl Projection for User {
    type Event = AuthEvent;

    fn apply(&mut self, _: &[Self::Event]) {
        // For the moment, we don't need to apply events to the projection.
    }
}
