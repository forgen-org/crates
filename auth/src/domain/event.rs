use super::scalar::{Email, Password, PasswordHash, UserId};
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub enum Event {
    Registered {
        at: DateTime<Utc>,
        email: Email,
        user_id: UserId,
    },
    PasswordChanged {
        at: DateTime<Utc>,
        password_hash: PasswordHash,
        user_id: UserId,
    },
    #[cfg(feature = "linkedin")]
    LinkedInConnected {
        access_token: String,
        at: DateTime<Utc>,
        refresh_token: String,
        user_id: UserId,
    },
    LoggedIn {
        at: DateTime<Utc>,
        user_id: UserId,
    },
}

pub trait State {
    fn is_already_registered(&self) -> Option<&UserId>;
    fn is_password_valid(&self, password: &Password) -> Option<&UserId>;
}

impl State for &[Event] {
    fn is_already_registered(&self) -> Option<&UserId> {
        self.iter()
            .filter_map(|event| match event {
                Event::Registered { user_id, .. } => Some(user_id),
                _ => None,
            })
            .last()
    }

    fn is_password_valid(&self, password: &Password) -> Option<&UserId> {
        let (password_hash, user_id) = self
            .iter()
            .filter_map(|event| match event {
                Event::PasswordChanged {
                    password_hash,
                    user_id,
                    ..
                } => Some((password_hash, user_id)),
                _ => None,
            })
            .last()?;

        if password_hash.verify(password) {
            Some(user_id)
        } else {
            None
        }
    }
}
