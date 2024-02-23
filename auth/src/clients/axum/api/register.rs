use crate::{
    application::{
        command::Register,
        port::*,
        query::GetJwtByUserId,
        scalar::{Email, Password},
    },
    clients::axum::{listener::wait_for_user, runtime::SignalSub},
    scalar::TransactionId,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use forgen::*;
use serde::Deserialize;
use std::sync::Arc;

pub async fn handler<R>(
    State(runtime): State<Arc<R>>,
    Json(payload): Json<Payload>,
) -> Result<String, Response>
where
    R: EventStore + JwtPort + SignalPub + SignalSub + UserRepository,
    R: Send + Sync,
{
    let mut command = Register::try_from(payload)?;
    command.transaction_id = Some(TransactionId::default());

    command
        .execute(runtime.as_ref())
        .await
        .map_err(|err| (StatusCode::UNAUTHORIZED, format!("{}", err)).into_response())?;

    let user_id = wait_for_user(runtime.as_ref(), command.transaction_id).await;

    let query = GetJwtByUserId { user_id };

    let jwt = query
        .fetch(runtime.as_ref())
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)).into_response())?;

    Ok(jwt.0)
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
            transaction_id: None,
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
