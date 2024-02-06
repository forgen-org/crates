use super::error::ApiError;
use crate::application::auth_command::{LoginCommand, RegisterCommand};
use crate::application::auth_port::*;
use crate::application::auth_query::GetJwtByEmail;
use axum::{extract::State, Json};
use framework::*;
use std::sync::Arc;

pub async fn login<R>(
    State(runtime): State<Arc<R>>,
    Json(command): Json<LoginCommand>,
) -> Result<Json<Jwt>, ApiError>
where
    R: Runtime + AuthStore + JwtPort + UserRepository,
{
    command.execute(runtime.as_ref()).await?;

    let jwt = GetJwtByEmail {
        email: command.email,
    }
    .execute(runtime.as_ref())
    .await?;

    Ok(Json(jwt))
}

pub async fn register<R>(
    State(runtime): State<Arc<R>>,
    Json(command): Json<RegisterCommand>,
) -> Result<Json<Jwt>, ApiError>
where
    R: Runtime + AuthStore + JwtPort + UserRepository,
{
    command.execute(runtime.as_ref()).await?;

    let jwt = GetJwtByEmail {
        email: command.email,
    }
    .execute(runtime.as_ref())
    .await?;

    Ok(Json(jwt))
}
