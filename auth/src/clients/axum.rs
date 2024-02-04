use crate::application::auth_command::{AuthCommandError, LoginCommand, RegisterCommand};
use crate::application::auth_port::*;
use crate::application::auth_query::{GetJwtByEmail, GetJwtByEmailError};
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
    R: Runtime + AuthStore + JwtPort + UserRepository + 'static,
{
    // let runtime = "test";
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(Arc::new(runtime))
}

async fn login<R>(
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

async fn register<R>(
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

#[derive(Debug, Error)]
enum ApiError {
    #[error(transparent)]
    AuthCommandError(#[from] AuthCommandError),
    #[error(transparent)]
    GetJwtByEmailError(#[from] GetJwtByEmailError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::AuthCommandError(err) => {
                (StatusCode::BAD_REQUEST, format!("{}", err)).into_response()
            }
            ApiError::GetJwtByEmailError(err) => {
                (StatusCode::BAD_REQUEST, format!("{}", err)).into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::test_runtime::TestRuntime;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::json;

    #[tokio::test]
    async fn test_register() {
        // let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        // let addr = listener.local_addr().unwrap();
        let runtime = TestRuntime::default();
        let app = auth_routes(runtime);

        let server = TestServer::new(app).unwrap();

        // Get the request.
        let response = server
            .post("/register")
            .json(&json!({
                "email": "email@example.com",
                "password": "password",
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
