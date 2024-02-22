use crate::{
    application::projection::User,
    scalar::{Email, UserId},
};
use forgen::UnexpectedError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserDto {
    email: String,
    user_id: String,
}

impl TryFrom<UserDto> for User {
    type Error = UnexpectedError;

    fn try_from(dto: UserDto) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Some(Email::parse(dto.email).map_err(UnexpectedError::from)?),
            user_id: Some(UserId::parse(&dto.user_id).map_err(UnexpectedError::from)?),
        })
    }
}

impl From<&User> for UserDto {
    fn from(projection: &User) -> Self {
        Self {
            email: projection
                .email
                .as_ref()
                .map(|email| email.to_string())
                .unwrap_or_default(),
            user_id: projection
                .user_id
                .as_ref()
                .map(|user_id| user_id.to_string())
                .unwrap_or_default(),
        }
    }
}
