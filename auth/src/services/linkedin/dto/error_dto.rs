use forgen::*;
use serde::Deserialize;

#[derive(Error, Debug, Deserialize)]
#[error("LinkedIn Error: {error}")]
pub struct ErrorDto {
    error: String,
    error_description: String,
}

#[derive(Error, Debug, Deserialize)]
#[error("LinkedIn Error: {message}")]
#[serde(rename_all = "camelCase")]
pub struct MessageDto {
    pub message: String,
    pub service_error_code: i32,
    pub status: i32,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ResultDto<T> {
    Ok(T),
    Error(ErrorDto),
    Message(MessageDto),
}

impl<T> ResultDto<T> {
    pub fn result(self) -> Result<T, UnexpectedError> {
        match self {
            ResultDto::Ok(value) => Ok(value),
            ResultDto::Error(error) => Err(UnexpectedError::from(error)),
            ResultDto::Message(message) => Err(UnexpectedError::from(message)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct TestDto {
        message: String,
    }

    #[test]
    fn test_error_dto() {
        let data = r#"
            {
                "error": "invalid_request",
                "error_description": "Unable to retrieve access token: appid/redirect uri/code verifier does not match authorization code. Or authorization code expired. Or external member binding exists"
            }
        "#;

        let result: Result<ErrorDto, _> = serde_json::from_str(data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_message_dto() {
        let data = r#"
            {
                "message": "Not enough permissions to access: GET-members /emailAddress",
                "serviceErrorCode": 100,
                "status": 403
            }
        "#;

        let result: Result<MessageDto, _> = serde_json::from_str(data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_result_dto() {
        let data = r#"
            {
                "error": "invalid_request",
                "error_description": "Unable to retrieve access token: appid/redirect uri/code verifier does not match authorization code. Or authorization code expired. Or external member binding exists"
            }
        "#;

        let result: ResultDto<TestDto> = serde_json::from_str(data).unwrap();

        assert!(matches!(result, ResultDto::Error(_)));
    }
}
