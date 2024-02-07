use super::{
    auth_event::{AuthEvent, Credentials},
    auth_scalar::{PasswordHash, UserId},
};

#[derive(Default)]
pub struct AuthState<'a> {
    pub is_registered: bool,
    pub password_hash: Option<&'a PasswordHash>,
    pub user_id: Option<&'a UserId>,
}

impl<'a> AuthState<'a> {
    pub fn new(events: &'a [AuthEvent]) -> Self {
        let mut state = AuthState::default();
        for event in events {
            match event {
                AuthEvent::Registered {
                    credentials,
                    user_id,
                    ..
                } => {
                    state.is_registered = true;
                    state.password_hash = match credentials {
                        Credentials::EmailPassword { password, .. } => Some(password),
                    };
                    state.user_id = Some(user_id);
                }
                _ => {}
            }
        }
        state
    }
}
