use crate::{
    application::{
        command::Login,
        port::*,
        query::GetJwtByUserId,
        scalar::{Email, Password},
    },
    transaction::Transaction,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use framework::*;
use futures::StreamExt;
use serde::Deserialize;
use std::sync::Arc;

pub async fn handler<R>(
    State(runtime): State<Arc<R>>,
    Json(payload): Json<Payload>,
) -> Result<String, Response>
where
    R: EventBus + EventStore + JwtPort + UserRepository + TransactionBus,
    R: Send + Sync,
{
    let command = Login::try_from(payload)?;

    let transaction_id = command
        .execute(runtime.as_ref())
        .await
        .map_err(|err| (StatusCode::UNAUTHORIZED, format!("{}", err)).into_response())?;

    let user_id = loop {
        if let Some(transaction) = TransactionBus::subscribe(runtime.as_ref()).next().await {
            let Transaction::UserProjected { id, user_id } = transaction;
            if id == transaction_id {
                break user_id;
            }
        }
    };

    let query = GetJwtByUserId { user_id };

    let jwt = query
        .fetch(runtime.as_ref())
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)).into_response())?;

    Ok(jwt.0)
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
