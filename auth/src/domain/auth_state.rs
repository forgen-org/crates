use super::{
    auth_event::{AuthEvent, Credentials},
    auth_scalar::{Password, PasswordHash, UserId},
};

pub struct AuthState<'a>(pub &'a [AuthEvent]);

impl<'a> AuthState<'a> {
    pub fn iter(&self) -> std::slice::Iter<AuthEvent> {
        self.0.iter()
    }

    pub fn is_registered(&self) -> bool {
        return self
            .iter()
            .any(|event| matches!(event, AuthEvent::Registered { .. }));
    }

    pub fn verify_password(&self, password: &Password) -> bool {
        let password_hash: &PasswordHash = &password.into();
        let current_password_hash = self
            .iter()
            .filter_map(|event| match event {
                AuthEvent::Registered { credentials, .. } => Some(credentials),
                _ => None,
            })
            .map(|credentials| match credentials {
                Credentials::EmailPassword { password, .. } => password_hash,
            })
            .last();

        match current_password_hash {
            Some(current_password_hash) => current_password_hash == password_hash,
            None => false,
        }
    }

    pub fn user_id(&self) -> Option<&UserId> {
        self.iter()
            .filter_map(|event| match event {
                AuthEvent::Registered { user_id, .. } => Some(user_id),
                _ => None,
            })
            .last()
    }
}
