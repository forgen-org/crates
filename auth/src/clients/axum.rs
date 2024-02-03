use crate::application::auth_command::{Register, RegisterError};
use crate::application::auth_port::*;
use crate::domain::auth_message::RegisterMethod;
use crate::domain::auth_scalar::Password;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use framework::*;
use serde::{Deserialize, Serialize};
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
    Json(dto): Json<RegisterDto>,
) -> Result<(), RegisterError>
where
    R: Runtime + AuthStore + UserRepository,
{
    let register: Register = dto.try_into().unwrap();
    register.execute(runtime.as_ref()).await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct RegisterDto {
    email: String,
    password: String,
}

impl TryFrom<RegisterDto> for Register {
    type Error = ();
    fn try_from(dto: RegisterDto) -> Result<Self, Self::Error> {
        Ok(Register(RegisterMethod::EmailPassword(
            Email::parse(&dto.email).unwrap(),
            Password(dto.password),
        )))
    }
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, format!("{}", self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::test_runtime::TestRuntime,
        domain::{auth_message::RegisterMethod, auth_scalar::Password},
    };
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use axum_test::TestServer;
    use serde_json::{json, Value};
    use std::{
        collections::HashMap,
        net::{SocketAddr, TcpListener},
    };

    #[tokio::test]
    async fn the_real_deal() {
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
