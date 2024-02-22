use super::api::{login, register};
use super::runtime::SignalSub;
use crate::application::port::*;
use crate::command::ProjectUser;
use crate::signal::Signal;
use axum::{routing::post, Router};
use forgen::*;
use std::sync::Arc;

pub struct AuthRouter;

impl AuthRouter {
    pub fn new<R>(runtime: Arc<R>) -> Router
    where
        R: EventStore + JwtPort + SignalSub + SignalPub + UserRepository,
        R: Send + Sync + 'static,
    {
        tokio::spawn({
            let runtime = runtime.clone();
            async move {
                let mut receiver = SignalSub::subscribe(runtime.as_ref());
                loop {
                    match receiver.recv().await.unwrap() {
                        Signal::EventsEmitted {
                            events,
                            transaction_id,
                            user_id,
                        } => {
                            if let Some(user_id) = user_id {
                                ProjectUser {
                                    events: events.clone(),
                                    user_id: user_id.clone(),
                                    transaction_id: transaction_id.clone(),
                                }
                                .execute(runtime.as_ref())
                                .unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            }
        });

        Router::new()
            .route("/login", post(login::handler))
            .route("/register", post(register::handler))
            // .nest("/linkedin", linkedin::router(runtime.clone()))
            .with_state(runtime)
    }
}
