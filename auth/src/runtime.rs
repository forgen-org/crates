use forgen::*;

#[derive(Delegate)]
pub struct Runtime {
    #[to(JwtPort)]
    pub jwt: crate::services::jwt::JwtService,

    #[cfg(feature = "linkedin")]
    #[to(LinkedInPort)]
    pub linkedin: crate::services::linkedin::LinkedInService,

    #[cfg(feature = "mongodb")]
    #[to(EventStore, UserRepository)]
    pub mongodb: crate::services::mongodb::MongoDbService,

    #[cfg(feature = "tokio")]
    #[to(SignalPub, TransactionBus)]
    pub tokio: crate::services::tokio::Tokio,
}

impl Runtime {
    pub async fn new() -> Self {
        Self {
            jwt: crate::services::jwt::JwtService::default(),
            #[cfg(feature = "linkedin")]
            linkedin: crate::services::linkedin::LinkedInService::default(),
            #[cfg(feature = "mongodb")]
            mongodb: crate::services::mongodb::MongoDbService::new().await,
            #[cfg(feature = "tokio")]
            tokio: crate::services::tokio::Tokio::default(),
        }
    }
}
