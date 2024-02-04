use crate::application::auth_port::*;
use framework::*;
use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ReplaceOptions},
    Client, Collection,
};
pub struct MongoDbService {
    event: Collection<AuthEvent>,
    user: Collection<User>,
}

impl MongoDbService {
    pub async fn new() -> Self {
        let client_options = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("auth");

        Self {
            event: db.collection("auth_events"),
            user: db.collection("auth_user"),
        }
    }
}

#[async_trait]
impl AuthStore for MongoDbService {
    async fn pull_by_email(&self, email: &Email) -> Result<Vec<AuthEvent>, AuthStoreError> {
        self.event
            .find(doc! {"email": email.to_string()}, None)
            .await
            .map_err(|_| AuthStoreError::DatabaseError)?
            .try_collect()
            .await
            .map_err(|_| AuthStoreError::DatabaseError)
    }
    async fn pull_by_user_id(&self, user_id: &UserId) -> Result<Vec<AuthEvent>, AuthStoreError> {
        self.event
            .find(doc! {"user_id": user_id.to_string()}, None)
            .await
            .map_err(|_| AuthStoreError::DatabaseError)?
            .try_collect()
            .await
            .map_err(|_| AuthStoreError::DatabaseError)
    }

    async fn push(&self, events: &[AuthEvent]) -> Result<(), AuthStoreError> {
        self.event
            .insert_many(events, None)
            .await
            .map_err(|_| AuthStoreError::DatabaseError)?;
        Ok(())
    }
}

#[async_trait]
impl UserRepository for MongoDbService {
    async fn find_by_email(&self, email: &Email) -> Result<User, UserRepositoryError> {
        self.user
            .find_one(doc! {"email": email.to_string()}, None)
            .await
            .map_err(|_| UserRepositoryError::UserNotFound)?
            .ok_or(UserRepositoryError::UserNotFound)
    }
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<User, UserRepositoryError> {
        self.user
            .find_one(doc! {"user_id": user_id.to_string()}, None)
            .await
            .map_err(|_| UserRepositoryError::UserNotFound)?
            .ok_or(UserRepositoryError::UserNotFound)
    }
    async fn save(&self, projection: &User) -> Result<(), UserRepositoryError> {
        self.user
            .replace_one(
                doc! {"user_id": projection.user_id.clone()},
                projection,
                ReplaceOptions::builder().upsert(true).build(),
            )
            .await
            .map_err(|_| UserRepositoryError::UserNotFound)?;
        Ok(())
    }
}
