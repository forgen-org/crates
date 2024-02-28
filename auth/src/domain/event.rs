use super::scalar::{Email, PasswordHash, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
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
        refresh_token: Option<String>,
        user_id: UserId,
    },
    LoggedIn {
        user_id: UserId,
    },
}
