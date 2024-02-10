use framework::*;

#[derive(Delegate)]
pub struct Runtime {
    #[cfg(feature = "mongodb")]
    #[to(EventStore, UserRepository)]
    mongodb_service: crate::services::mongodb::MongoDbService,

    #[to(EventBus, TransactionBus)]
    membus: crate::services::membus::MemBus,

    #[to(JwtPort)]
    jwt_service: crate::services::jwt::JwtService,
}

impl Runtime {
    pub async fn new(jwt_secret: &str) -> Self {
        Self {
            #[cfg(feature = "mongodb")]
            mongodb_service: crate::services::mongodb::MongoDbService::new().await,

            membus: crate::services::membus::MemBus::default(),

            jwt_service: crate::services::jwt::JwtService::new(jwt_secret),
        }
    }
}
