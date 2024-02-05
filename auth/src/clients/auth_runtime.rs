use framework::*;

#[derive(Delegate)]
pub struct AuthRuntime {
    #[cfg(feature = "mongodb")]
    #[to(AuthStore, UserRepository)]
    mongodb_service: crate::services::mongodb::MongoDbService,

    #[to(JwtPort)]
    jwt_service: crate::services::jwt::JwtService,
}

impl AuthRuntime {
    pub async fn new(jwt_secret: &str) -> Self {
        Self {
            #[cfg(feature = "mongodb")]
            mongodb_service: crate::services::mongodb::MongoDbService::new().await,
            jwt_service: crate::services::jwt::JwtService::new(jwt_secret),
        }
    }
}

impl Runtime for AuthRuntime {}
