use crate::application::port::*;
use crate::signal::Signal;
use forgen::*;
use tokio::sync::broadcast::Receiver;

#[derive(Default, Delegate)]
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
