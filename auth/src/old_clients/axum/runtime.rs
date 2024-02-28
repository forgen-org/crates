use crate::application::port::*;
use crate::signal::Signal;
use forgen::*;
use tokio::sync::broadcast::Receiver;

#[derive(Delegate)]
pub struct Runtime {
    #[to(JwtPort)]
    jwt_service: crate::services::jwt::JwtService,

    #[cfg(feature = "linkedin")]
    #[to(LinkedInPort)]
    linkedin_service: crate::services::linkedin::LinkedInService,

    #[cfg(feature = "mongodb")]
    #[to(EventStore, UserRepository)]
    mongodb_service: crate::services::mongodb::MongoDbService,

    #[to(SignalPub, TransactionBus)]
    tokiobus: crate::services::tokiobus::TokioBus,
}

impl Runtime {
    pub async fn new() -> Self {
        Self {
            jwt_service: crate::services::jwt::JwtService::default(),
            #[cfg(feature = "linkedin")]
            linkedin_service: crate::services::linkedin::LinkedInService::default(),
            #[cfg(feature = "mongodb")]
            mongodb_service: crate::services::mongodb::MongoDbService::new().await,
            tokiobus: crate::services::tokiobus::TokioBus::default(),
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

#[cfg(not(feature = "linkedin"))]
pub trait Ports: EventStore + JwtPort + SignalSub + SignalPub + UserRepository {}

#[cfg(feature = "linkedin")]
pub trait Ports:
    EventStore + JwtPort + SignalSub + SignalPub + UserRepository + LinkedInPort
{
}

impl Ports for Runtime {}
