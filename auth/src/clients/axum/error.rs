use crate::application::auth_command::AuthCommandError;
use crate::application::auth_query::GetJwtByEmailError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use framework::*;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Invalid JWT")]
    InvalidJwt,

    #[error(transparent)]
    AuthCommandError(#[from] AuthCommandError),
    #[error(transparent)]
    GetJwtByEmailError(#[from] GetJwtByEmailError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::InvalidJwt => (StatusCode::UNAUTHORIZED, "Invalid JWT").into_response(),
            ApiError::AuthCommandError(err) => {
                (StatusCode::BAD_REQUEST, format!("{}", err)).into_response()
            }
            ApiError::GetJwtByEmailError(err) => {
                (StatusCode::BAD_REQUEST, format!("{}", err)).into_response()
            }
        }
    }
}
