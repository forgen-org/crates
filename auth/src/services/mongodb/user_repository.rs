use super::user_dto::UserDto;
use super::MongoDbService;
use crate::application::{port::UserRepository, projection::User, scalar::UserId};
use forgen::*;
use mongodb::{bson::doc, options::ReplaceOptions};

#[async_trait]
impl UserRepository for MongoDbService {
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError> {
        let dto = self
            .user
            .find_one(doc! {"user_id": user_id.to_string()}, None)
            .await
            .map_err(UnexpectedError::from)?;

        match dto {
            Some(dto) => Ok(Some(User::try_from(dto)?)),
            None => Ok(None),
        }
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
                UserDto::from(projection),
                ReplaceOptions::builder().upsert(true).build(),
            )
            .await
            .map(|_| ())
            .map_err(UnexpectedError::from)
    }
}
