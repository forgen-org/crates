use super::scalar::{Email, PasswordHash};

#[derive(Clone)]
pub enum Event {
    Registered {
        email: Email,
    },
    PasswordChanged {
        password_hash: PasswordHash,
    },
    #[cfg(feature = "linkedin")]
    LinkedInConnected {
        access_token: String,
        refresh_token: String,
    },
    LoggedIn,
}
