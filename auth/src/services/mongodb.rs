use crate::application::{port::*, projection::User};
use crate::domain::{
    event::Event,
    scalar::{Email, UserId},
};
use forgen::*;
use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ReplaceOptions},
    Client, Collection,
};

pub struct MongoDbService {
    pub(crate) event: Collection<Event>,
    pub(crate) user: Collection<User>,
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

#[async_trait]
impl EventStore for MongoDbService {
    async fn identify_by_email(&self, email: &Email) -> Result<Option<UserId>, UnexpectedError> {
        let event_option = self
            .event
            .find_one(
                doc! {"_tag": "Registered", "email": email.to_string()},
                None,
            )
            .await
            .map_err(UnexpectedError::from)?;

        match event_option {
            Some(event) => {
                match Event::try_from(event) {
                    // Assuming Event::try_from is replaced with TryInto trait for cleaner syntax
                    Ok(Event::Registered { user_id, .. }) => Ok(Some(user_id)),
                    Ok(_) => Err(UnexpectedError::from(
                        "Found event is not a Registered event",
                    )),
                    Err(e) => Err(UnexpectedError::from(e)), // Handle conversion error
                }
            }
            None => Ok(None),
        }
    }

    async fn pull_by_user_id(&self, user_id: &UserId) -> Result<Vec<Event>, UnexpectedError> {
        self.event
            .find(doc! {"user_id": user_id.to_string()}, None)
            .await
            .map_err(UnexpectedError::from)?
            .try_collect()
            .await
            .map_err(UnexpectedError::from)
    }

    async fn push(&self, events: &[Event]) -> Result<(), UnexpectedError> {
        self.event
            .insert_many(events, None)
            .await
            .map(|_| ())
            .map_err(UnexpectedError::from)
    }
}

#[async_trait]
impl UserRepository for MongoDbService {
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError> {
        self.user
            .find_one(doc! {"user_id": user_id.to_string()}, None)
            .await
            .map_err(UnexpectedError::from)
    }

    async fn save(&self, projection: &User) -> Result<(), UnexpectedError> {
        let user_id = projection
            .user_id
            .as_ref()
            .map(|user_id| user_id.to_string())
            .unwrap_or_default();

        self.user
            .replace_one(
                doc! {"user_id": user_id },
                projection,
                ReplaceOptions::builder().upsert(true).build(),
            )
            .await
            .map(|_| ())
            .map_err(UnexpectedError::from)
    }
}