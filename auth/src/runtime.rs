use forgen::*;

pub struct RuntimeConfig {
    #[cfg(feature = "linkedin")]
    pub linkedin: crate::services::linkedin::LinkedInConfig,
}

#[derive(Delegate)]
pub struct Runtime {
    #[cfg(feature = "jwt")]
    #[to(JwtPort)]
    pub jwt: crate::services::jwt::JwtService,

    #[cfg(feature = "linkedin")]
    #[to(LinkedInApi)]
    pub linkedin: crate::services::linkedin::LinkedInService,

    #[cfg(feature = "mongodb")]
    #[to(EventStore, UserRepository)]
    pub mongodb: crate::services::mongodb::MongoDbService,

    #[cfg(feature = "tokio")]
    #[to(SignalPub, TransactionBus)]
    pub tokio: crate::services::tokio::Tokio,

    #[cfg(feature = "web_sys")]
    #[to(JwtStore, WebView)]
    pub web_sys: crate::services::web_sys::WebSys,
}

impl Runtime {
    pub fn new(_config: RuntimeConfig) -> Self {
        Self {
            #[cfg(feature = "jwt")]
            jwt: crate::services::jwt::JwtService::default(),

            #[cfg(feature = "linkedin")]
            linkedin: crate::services::linkedin::LinkedInService::new(_config.linkedin),

            #[cfg(feature = "mongodb")]
            mongodb: crate::services::mongodb::MongoDbService::default(),

            #[cfg(feature = "tokio")]
            tokio: crate::services::tokio::Tokio::default(),

            #[cfg(feature = "web_sys")]
            web_sys: crate::services::web_sys::WebSys::default(),
        }
    }
}
