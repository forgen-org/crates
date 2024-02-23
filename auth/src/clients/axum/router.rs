#[cfg(feature = "linkedin")]
use super::api::linkedin;
use super::api::{login, register};
use super::runtime::{Ports, SignalSub};
use crate::command::ProjectUser;
use crate::signal::Signal;
use axum::routing::get;
use axum::{routing::post, Router};
use forgen::*;
use std::sync::Arc;

pub struct AuthRouter;

impl AuthRouter {
    pub fn new<R>(runtime: Arc<R>) -> Router
    where
        R: Ports,
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
                                .await
                                .unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            }
        });

        let mut router = Router::new()
            .route("/login", post(login::handler))
            .route("/register", post(register::handler));

        #[cfg(feature = "linkedin")]
        {
            router = router.nest(
                "/linkedin",
                Router::new()
                    .with_state(runtime.clone())
                    .route("/callback", get(linkedin::callback::handler)),
            );
        }

        router.with_state(runtime.clone())
    }
}
