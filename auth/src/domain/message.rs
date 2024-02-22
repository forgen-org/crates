use super::scalar::*;

pub enum Message {
    Register {
        email: Email,
        password: Password,
    },
    LogIn {
        email: Email,
        password: Password,
    },
    #[cfg(feature = "linkedin")]
    ConnectLinkedIn {
        email: Email,
        access_token: String,
        refresh_token: String,
    },
}
