use crate::domain::auth_scalar::{Email, PasswordHash};
use framework::Event;
use serde::{Deserialize, Serialize};

use super::auth_scalar::UserId;

#[derive(Serialize, Deserialize)]
pub enum AuthEvent {
    Registered(UserId, Credentials),
    EmailValidated(UserId),
    LoggedIn(UserId),
}

#[derive(Serialize, Deserialize)]
pub enum Credentials {
    EmailPassword(Email, PasswordHash),
}

impl Event for AuthEvent {}
