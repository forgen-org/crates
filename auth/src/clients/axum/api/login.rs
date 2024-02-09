use crate::application::*;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use framework::*;
use serde::Deserialize;
use std::sync::Arc;

pub async fn handler<R>(
    State(runtime): State<Arc<R>>,
    Json(payload): Json<Payload>,
) -> Result<Json<Jwt>, Response>
where
    R: Framework,
    R: AuthStore + JwtPort + UserRepository,
{
    let command = Login::try_from(payload)?;
    let email = command.email.clone();

    runtime
        .execute(command)
        .await
        .map_err(|err| (StatusCode::UNAUTHORIZED, format!("{}", err)).into_response())?;

    let jwt = runtime
        .fetch(GetJwtByEmail { email })
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)).into_response())?;

    Ok(Json(jwt))
}

#[derive(Deserialize)]
pub struct Payload {
    email: String,
    password: String,
}

impl TryFrom<Payload> for Login {
    type Error = Response;

    fn try_from(payload: Payload) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Email::parse(payload.email)
                .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid email").into_response())?,
            password: Password::parse(payload.password)
                .map_err(|err| (StatusCode::BAD_REQUEST, format!("{}", err)).into_response())?,
        })
    }
}
