use crate::domain::scalar::Email;
use forgen::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInfoDto {
    pub email_verified: bool,
    pub email: String,
    pub family_name: String,
    pub given_name: String,
    pub locale: LocaleDto,
    pub name: String,
    pub picture: String,
    pub sub: String,
}

#[derive(Deserialize)]
pub struct LocaleDto {
    pub country: String,
    pub language: String,
}

impl TryFrom<UserInfoDto> for Email {
    type Error = UnexpectedError;

    fn try_from(value: UserInfoDto) -> Result<Self, Self::Error> {
        Email::parse(&value.email).map_err(UnexpectedError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_info_dto() {
        let data = r#"
            {
                "localizedLastName": "Doe",
                "localizedFirstName": "John",
                "id": "123456789",
                "email": "
            }
        "#;

        let result: Result<UserInfoDto, _> = serde_json::from_str(data);

        assert!(result.is_ok());
    }
}
