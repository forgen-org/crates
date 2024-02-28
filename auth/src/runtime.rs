use forgen::*;

#[derive(Default, Delegate)]
pub struct Runtime {
    #[cfg(feature = "jwt")]
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

    #[cfg(feature = "web_sys")]
    #[to(JwtStore, WebView)]
    pub web_sys: crate::services::web_sys::WebSys,
}
