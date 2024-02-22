use super::user_dto::UserDto;
use super::MongoDbService;
use crate::application::{port::UserRepository, projection::User, scalar::UserId};
use forgen::*;
use mongodb::{bson::doc, options::ReplaceOptions};

impl UserRepository for MongoDbService {
    fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError> {
        self.user
            .find_one(doc! {"user_id": user_id.to_string()}, None)
            .map(|dto| dto.map(User::from))
            .map_err(UnexpectedError::from)
    }

    fn save(&self, projection: &User) -> Result<(), UnexpectedError> {
        self.user
            .replace_one(
                doc! {"user_id": projection.user_id.clone()},
                UserDto::from(projection),
                ReplaceOptions::builder().upsert(true).build(),
            )
            .map(|_| ())
            .map_err(UnexpectedError::from)
    }
}
