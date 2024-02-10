use crate::application::projection::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserDto {
    email: String,
    user_id: String,
}

impl From<UserDto> for User {
    fn from(dto: UserDto) -> Self {
        Self {
            email: dto.email,
            user_id: dto.user_id,
        }
    }
}

impl From<&User> for UserDto {
    fn from(projection: &User) -> Self {
        Self {
            email: projection.email.clone(),
            user_id: projection.user_id.clone(),
        }
    }
}
