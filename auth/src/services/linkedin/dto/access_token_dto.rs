use crate::application::port::LinkedInTokens;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccessTokenDto {
    access_token: String,
    refresh_token: Option<String>,
}

impl From<AccessTokenDto> for LinkedInTokens {
    fn from(response: AccessTokenDto) -> Self {
        LinkedInTokens {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_token_dto() {
        let data = r#"
            {
                "access_token": "AQVb",
                "expires_in": 5183999,
                "scope": "email,openid,profile",
                "token_type": "Bearer",
                "id_token": "eyJ6aXAiOi"
            }
        "#;

        let result: Result<AccessTokenDto, _> = serde_json::from_str(data);

        assert!(result.is_ok());
    }
}
