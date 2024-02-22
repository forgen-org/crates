use crate::scalar::UserId;

use super::scalar::{Email, PasswordHash};

#[derive(Clone)]
pub enum Event {
    Registered {
        email: Email,
        user_id: UserId,
    },
    PasswordChanged {
        password_hash: PasswordHash,
        user_id: UserId,
    },
    #[cfg(feature = "linkedin")]
    LinkedInConnected {
        access_token: String,
        refresh_token: String,
        user_id: UserId,
    },
    LoggedIn {
        user_id: UserId,
    },
}
