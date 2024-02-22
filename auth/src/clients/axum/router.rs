use super::api::{login, register};
use crate::{application::port::*, listener::RecomputeUser, signal::Signal};
use axum::{routing::post, Router};
use forgen::*;
use std::sync::Arc;

pub struct AuthRouter;

impl AuthRouter {
    pub fn new<R>(runtime: Arc<R>) -> Router
    where
        R: SignalBus + EventStore + JwtPort + UserRepository,
        R: Send + Sync + 'static,
    {
        SignalBus::subscribe(runtime.as_ref(), {
            info!("Subscribing to RecomputeUser");
            let runtime = runtime.clone();
            move |signal| {
                info!("RecomputeUser");
                RecomputeUser(signal).listen(runtime.as_ref());
            }
        });

        Router::new()
            .route("/login", post(login::handler))
            .route("/register", post(register::handler))
            // .nest("/linkedin", linkedin::router(runtime.clone()))
            .with_state(runtime)
    }
}
