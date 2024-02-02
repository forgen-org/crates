use crate::domain::auth_scalar::{Email, PasswordHash};
use framework::Event;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum AuthEvent {
    Registered(Credentials),
    EmailValidated,
    LoggedIn,
}

#[derive(Serialize, Deserialize)]
pub enum Credentials {
    EmailPassword(Email, PasswordHash),
}

impl Event for AuthEvent {}
