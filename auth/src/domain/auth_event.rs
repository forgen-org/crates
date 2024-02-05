use super::auth_scalar::UserId;
use crate::domain::auth_scalar::{Email, PasswordHash};
use chrono::{DateTime, Utc};
use framework::Event;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "_tag")]
pub enum AuthEvent {
    Registered {
        at: DateTime<Utc>,
        credentials: Credentials,
        user_id: UserId,
    },
    EmailValidated {
        at: DateTime<Utc>,
        user_id: UserId,
    },
    LoggedIn {
        at: DateTime<Utc>,
        user_id: UserId,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "_tag")]
pub enum Credentials {
    EmailPassword {
        email: Email,
        password: PasswordHash,
    },
}

impl Event for AuthEvent {}
