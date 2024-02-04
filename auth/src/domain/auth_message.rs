use crate::domain::{
    auth_event::{AuthEvent, Credentials},
    auth_scalar::{Email, Password},
};
use framework::*;
use serde::Deserialize;

pub enum AuthMessage {
    Register(RegisterMethod),
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

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Already registered")]
    AlreadyRegistered,
}

impl Message<AuthEvent, AuthError> for AuthMessage {
    fn send(self, events: &[AuthEvent]) -> Result<Vec<AuthEvent>, AuthError> {
        match self {
            AuthMessage::Register(credentials) => {
                if events
                    .iter()
                    .any(|event| matches!(event, AuthEvent::Registered(_)))
                {
                    Err(AuthError::AlreadyRegistered)
                } else {
                    Ok(vec![AuthEvent::Registered(credentials.into())])
                }
            }
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

        let events = vec![AuthEvent::Registered(Credentials::EmailPassword(
            Email::parse("email@example.com").unwrap(),
            Password::parse("12345").unwrap().into(),
        ))];
        let res = AuthMessage::Register(credentials).send(&events);

        assert!(matches!(res, Err(AuthError::AlreadyRegistered)));
    }
}
