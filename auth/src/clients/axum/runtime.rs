use crate::signal::Signal;
use forgen::*;
use tokio::sync::broadcast::Receiver;

#[derive(Delegate)]
pub struct Runtime {
    #[cfg(feature = "mongodb")]
    #[to(EventStore, UserRepository)]
    mongodb_service: crate::services::mongodb::MongoDbService,

    #[to(SignalPub, TransactionBus)]
    tokiobus: crate::services::tokiobus::TokioBus,

    #[to(JwtPort)]
    jwt_service: crate::services::jwt::JwtService,
}

impl Runtime {
    pub async fn new(jwt_secret: &str) -> Self {
        Self {
            #[cfg(feature = "mongodb")]
            mongodb_service: crate::services::mongodb::MongoDbService::new(),

            #[cfg(feature = "axum")]
            tokiobus: crate::services::tokiobus::TokioBus::default(),

            jwt_service: crate::services::jwt::JwtService::new(jwt_secret),
        }
    }
}

pub trait SignalSub {
    fn subscribe(&self) -> Receiver<Signal>;
}

impl SignalSub for Runtime {
    fn subscribe(&self) -> Receiver<Signal> {
        self.tokiobus.subscribe()
    }
}
