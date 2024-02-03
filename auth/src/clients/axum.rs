use crate::application::auth_command::{Register, RegisterError};
use crate::application::auth_port::*;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use framework::*;
use std::sync::Arc;

pub fn auth_routes<R>(runtime: R) -> Router
where
    R: Runtime + AuthStore + UserRepository + 'static,
{
    // let runtime = "test";
    Router::new()
        .route("/register", post(register))
        .with_state(Arc::new(runtime))
}

async fn register<R>(
    State(runtime): State<Arc<R>>,
    Json(command): Json<Register>,
) -> Result<(), RegisterError>
where
    R: Runtime + AuthStore + UserRepository,
{
    command.execute(runtime.as_ref()).await?;
    Ok(())
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, format!("{}", self)).into_response()
    }
}
