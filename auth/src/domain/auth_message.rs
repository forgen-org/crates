use crate::domain::{
    auth_event::{AuthEvent, Credentials},
    auth_scalar::{Email, Password},
};
use framework::*;
use serde::Deserialize;

use super::{auth_scalar::UserId, auth_state::AuthState};

pub enum AuthMessage {
    Register(RegisterMethod),
    LogIn(RegisterMethod),
}

#[derive(Deserialize)]
pub enum RegisterMethod {
    EmailPassword(Email, Password),
}

impl From<RegisterMethod> for Credentials {
    fn from(method: RegisterMethod) -> Self {
        match method {
            RegisterMethod::EmailPassword(email, password) => {
                Credentials::EmailPassword(email, password.into())
            }
        }
    }
}

impl From<&RegisterMethod> for Credentials {
    fn from(method: &RegisterMethod) -> Self {
        method.to_owned().into()
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Already registered")]
    AlreadyRegistered,

    #[error("Invalid password")]
    InvalidPassword,
}

impl Message<AuthEvent, AuthError> for AuthMessage {
    fn send(&self, events: &[AuthEvent]) -> Result<Vec<AuthEvent>, AuthError> {
        let state = AuthState(events);
        match self {
            AuthMessage::Register(credentials) => {
                if state.is_registered() {
                    Err(AuthError::AlreadyRegistered)
                } else {
                    let user_id = UserId::default();
                    Ok(vec![AuthEvent::Registered(user_id, credentials.into())])
                }
            }
            AuthMessage::LogIn(_) => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_already_registered() {
        let credentials = RegisterMethod::EmailPassword(
            Email::parse("email@example.com").unwrap(),
            Password::parse("password").unwrap(),
        );

        let events = vec![AuthEvent::Registered(
            UserId::default(),
            Credentials::EmailPassword(
                Email::parse("email@example.com").unwrap(),
                Password::parse("12345678").unwrap().into(),
            ),
        )];
        let res = AuthMessage::Register(credentials).send(&events);

        assert!(matches!(res, Err(AuthError::AlreadyRegistered)));
    }
}
