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
    ConnectLinkedIn {
        email: Email,
        access_token: String,
        refresh_token: Option<String>,
    },
}
