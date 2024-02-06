use super::handler::*;
use crate::application::auth_port::*;
use axum::{routing::post, Router};
use framework::*;
use std::sync::Arc;

pub struct AuthRouter;

impl AuthRouter {
    pub fn new<R>(runtime: Arc<R>) -> Router
    where
        R: Runtime + AuthStore + JwtPort + UserRepository + 'static,
    {
        // let runtime = "test";
        Router::new()
            .route("/login", post(login))
            .route("/register", post(register))
            .with_state(runtime)
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
        let runtime = Arc::new(TestRuntime::default());
        let app = AuthRouter::new(runtime);

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
