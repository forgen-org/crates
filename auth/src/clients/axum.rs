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
    Json(register): Json<Register>,
) -> Result<(), RegisterError>
where
    R: Runtime + AuthStore + UserRepository,
{
    // let register: Register = dto.try_into().unwrap();
    register.execute(runtime.as_ref()).await?;
    Ok(())
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, format!("{}", self)).into_response()
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
