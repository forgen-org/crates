use framework::Projection;
use serde::{Deserialize, Serialize};

use crate::domain::{auth_event::AuthEvent, auth_scalar::UserId};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
}

impl Projection for User {
    type Event = AuthEvent;

    fn apply(&mut self, _: &[Self::Event]) {
        // For the moment, we don't need to apply events to the projection.
    }
}
