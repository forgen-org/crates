use super::event_dto::EventDto;
use super::user_dto::UserDto;
use mongodb::{options::ClientOptions, Client, Collection};

pub struct MongoDbService {
    pub(crate) event: Collection<EventDto>,
    pub(crate) user: Collection<UserDto>,
}

impl MongoDbService {
    pub async fn new() -> Self {
        let mongo_url = std::env::var("AUTH_MONGO_URL")
            .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        let client_options = ClientOptions::parse(mongo_url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("auth");

        Self {
            event: db.collection("auth_events"),
            user: db.collection("auth_user"),
        }
    }
}
