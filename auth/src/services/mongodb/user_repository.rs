use super::user_dto::UserDto;
use super::MongoDbService;
use crate::application::{
    port::UserRepository,
    projection::User,
    scalar::{Email, UserId},
};
use framework::*;
use mongodb::{bson::doc, options::ReplaceOptions};

#[async_trait]
impl UserRepository for MongoDbService {
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UnexpectedError> {
        self.user
            .find_one(doc! {"email": email.to_string()}, None)
            .await
            .map(|dto| dto.map(User::from))
            .map_err(UnexpectedError::from)
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError> {
        self.user
            .find_one(doc! {"user_id": user_id.to_string()}, None)
            .await
            .map(|dto| dto.map(User::from))
            .map_err(UnexpectedError::from)
    }

    async fn save(&self, projection: &User) -> Result<(), UnexpectedError> {
        self.user
            .replace_one(
                doc! {"user_id": projection.user_id.clone()},
                UserDto::from(projection),
                ReplaceOptions::builder().upsert(true).build(),
            )
            .await
            .map(|_| ())
            .map_err(UnexpectedError::from)
    }
}
