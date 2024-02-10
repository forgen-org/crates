use crate::application::{
    command::Register,
    port::*,
    query::GetJwtByEmail,
    scalar::{Email, Password},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use framework::*;
use serde::Deserialize;
use std::sync::Arc;
use tokio_retry::{strategy::ExponentialBackoff, Retry};

pub async fn handler<R>(
    State(runtime): State<Arc<R>>,
    Json(payload): Json<Payload>,
) -> Result<String, Response>
where
    R: Framework,
    R: EventBus + EventStore + JwtPort + UserRepository,
{
    let command = Register::try_from(payload)?;
    let email = command.email.clone();

    runtime
        .execute(command)
        .await
        .map_err(|err| (StatusCode::UNAUTHORIZED, format!("{}", err)).into_response())?;

    Retry::spawn(ExponentialBackoff::from_millis(100).take(8), || {
        runtime.fetch(GetJwtByEmail {
            email: email.clone(),
        })
    })
    .await
    .map(|jwt| jwt.0)
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)).into_response())
}

#[derive(Deserialize)]
pub struct Payload {
    pub email: String,
    pub password: String,
}

impl TryFrom<Payload> for Register {
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::application::test_runtime::TestRuntime;
//     use axum::http::StatusCode;
//     use axum_test::TestServer;
//     use serde_json::json;

//     #[tokio::test]
//     async fn test_register() {
//         // let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
//         // let addr = listener.local_addr().unwrap();
//         let runtime = Arc::new(TestRuntime::default());
//         let app = AuthRouter::new(runtime);

//         let server = TestServer::new(app).unwrap();

//         // Get the request.
//         let response = server
//             .post("/register")
//             .json(&json!({
//                 "email": "email@example.com",
//                 "password": "password",
//             }))
//             .await;

//         assert_eq!(response.status_code(), StatusCode::OK);
//     }
// }
