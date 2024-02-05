use crate::domain::{
    auth_event::{AuthEvent, Credentials},
    auth_scalar::{Email, Password},
};
use framework::*;
use serde::Deserialize;

use super::{auth_scalar::UserId, auth_state::AuthState};

pub enum AuthMessage {
    Register { method: RegisterMethod },
    LogIn { method: RegisterMethod },
}

#[derive(Deserialize)]
pub enum RegisterMethod {
    EmailPassword { email: Email, password: Password },
}

impl From<&RegisterMethod> for Credentials {
    fn from(method: &RegisterMethod) -> Self {
        match method {
            RegisterMethod::EmailPassword { email, password } => Credentials::EmailPassword {
                email: email.clone(),
                password: password.into(),
            },
        }
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Already registered")]
    AlreadyRegistered,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Not registered")]
    NotRegistered,
}

impl Message<AuthEvent, AuthError> for AuthMessage {
    fn send(&self, events: &[AuthEvent]) -> Result<Vec<AuthEvent>, AuthError> {
        let state = AuthState(events);
        match self {
            AuthMessage::Register { method } => {
                if state.is_registered() {
                    Err(AuthError::AlreadyRegistered)
                } else {
                    let user_id = UserId::default();
                    Ok(vec![AuthEvent::Registered {
                        at: chrono::Utc::now(),
                        user_id,
                        credentials: method.into(),
                    }])
                }
            }
            AuthMessage::LogIn { method } => {
                let RegisterMethod::EmailPassword { password, .. } = method;

                if let Some(user_id) = state.user_id() {
                    if state.verify_password(password) {
                        Ok(vec![AuthEvent::LoggedIn {
                            at: chrono::Utc::now(),
                            user_id: user_id.clone(),
                        }])
                    } else {
                        Err(AuthError::InvalidPassword)
                    }
                } else {
                    Err(AuthError::NotRegistered)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::*;

    #[test]
    fn test_already_registered() {
        let events = vec![AuthEvent::Registered {
            at: Utc.timestamp_opt(0, 0).unwrap(),
            credentials: Credentials::EmailPassword {
                email: Email::parse("email@example.com").unwrap(),
                password: Password::parse("12345678").unwrap().into(),
            },
            user_id: UserId::default(),
        }];

        let res = AuthMessage::Register {
            method: RegisterMethod::EmailPassword {
                email: Email::parse("email@example.com").unwrap(),
                password: Password::parse("password").unwrap(),
            },
        }
        .send(&events);

        assert!(matches!(res, Err(AuthError::AlreadyRegistered)));
    }
}
