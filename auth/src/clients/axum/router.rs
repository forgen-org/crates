use super::api::{login, register};
use crate::application::port::*;
use axum::{routing::post, Router};
use std::sync::Arc;

pub struct AuthRouter;

impl AuthRouter {
    pub fn new<R>(runtime: Arc<R>) -> Router
    where
        R: EventBus + EventStore + JwtPort + UserRepository + TransactionBus,
        R: Send + Sync + 'static,
    {
        Router::new()
            .route("/login", post(login::handler))
            .route("/register", post(register::handler))
            // .nest("/linkedin", linkedin::router(runtime.clone()))
            .with_state(runtime)
    }
}
