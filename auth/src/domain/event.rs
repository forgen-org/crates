use super::scalar::{Email, PasswordHash, UserId};
use chrono::{DateTime, Utc};

pub enum Event {
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

pub enum Credentials {
    EmailPassword {
        email: Email,
        password_hash: PasswordHash,
    },
}
