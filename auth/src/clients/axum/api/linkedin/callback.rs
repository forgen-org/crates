use crate::{
    application::{
        command::ConnectLinkedIn, port::*, query::GetJwtByUserId, scalar::TransactionId,
    },
    clients::axum::{listener::wait_for_user, runtime::SignalSub},
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use forgen::*;
use serde::Deserialize;
use std::sync::Arc;
use tokio::task;

pub async fn handler<R>(
    State(runtime): State<Arc<R>>,
    params: Query<Params>,
) -> Result<String, Response>
where
    R: EventStore + LinkedInPort + JwtPort + SignalPub + SignalSub + UserRepository,
    R: Send + Sync + 'static,
{
    let transaction_id = TransactionId::default();

    task::spawn_blocking({
        let runtime = runtime.clone();
        let transaction_id = transaction_id.clone();
        move || {
            ConnectLinkedIn {
                code: params.code.clone(),
                transaction_id: Some(transaction_id.clone()),
            }
            .execute(runtime.as_ref())
            .map_err(|err| (StatusCode::UNAUTHORIZED, format!("{}", err)).into_response())
        }
    })
    .await
    .unwrap()?;

    let user_id = wait_for_user(runtime.as_ref(), Some(transaction_id)).await;

    let jwt = GetJwtByUserId { user_id }
        .fetch(runtime.as_ref())
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)).into_response())?;

    Ok(jwt.0)
}

#[derive(Deserialize)]
pub struct Params {
    code: String,
}
