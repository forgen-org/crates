use crate::application::*;
use chrono::{DateTime, Utc};
use framework::*;
use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ReplaceOptions},
    Client, Collection,
};
use serde::{Deserialize, Serialize};

pub struct MongoDbService {
    event: Collection<EventDto>,
    user: Collection<UserDto>,
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
    async fn pull_by_email(&self, email: &Email) -> Result<Vec<Event>, UnexpectedError> {
        self.event
            .find(doc! {"credentials.email": email.to_string()}, None)
            .await
            .map_err(UnexpectedError::from)?
            // .map_err(UnexpectedError::into)?
            .try_collect()
            .await
            .map_err(UnexpectedError::from)
            .and_then(|events: Vec<EventDto>| {
                events
                    .into_iter()
                    .map(Event::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
    }
    async fn pull_by_user_id(&self, user_id: &UserId) -> Result<Vec<Event>, UnexpectedError> {
        self.event
            .find(doc! {"user_id": user_id.to_string()}, None)
            .await
            .map_err(UnexpectedError::from)?
            .try_collect()
            .await
            .map_err(UnexpectedError::from)
            .and_then(|events: Vec<EventDto>| {
                events
                    .into_iter()
                    .map(Event::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
    }

    async fn push(&self, events: &[Event]) -> Result<(), UnexpectedError> {
        self.event
            .insert_many(
                events
                    .iter()
                    .map(|event| EventDto::from(event))
                    .collect::<Vec<_>>(),
                None,
            )
            .await
            .map(|_| ())
            .map_err(UnexpectedError::from)
    }
}

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

// impl<T> From<T> for UnexpectedError
// where
//     T: std::fmt::Display,
// {
//     fn from(error: T) -> Self {
//         UnexpectedError::UnknownError(format!("MongoDbService: {:?}", error))
//     }
// }

#[derive(Serialize, Deserialize)]
#[serde(tag = "_tag")]
pub enum EventDto {
    Registered {
        at: DateTime<Utc>,
        email: String,
        password_hash: [u8; 32],
        user_id: String,
    },
    EmailValidated {
        at: DateTime<Utc>,
        user_id: String,
    },
    LoggedIn {
        at: DateTime<Utc>,
        user_id: String,
    },
}

impl TryFrom<EventDto> for Event {
    type Error = UnexpectedError;

    fn try_from(dto: EventDto) -> Result<Self, Self::Error> {
        Ok(match dto {
            EventDto::Registered {
                at,
                email,
                password_hash,
                user_id,
            } => Event::Registered {
                at,
                credentials: Credentials::EmailPassword {
                    email: Email::parse(email).map_err(UnexpectedError::from)?,
                    password_hash: PasswordHash(password_hash),
                },
                user_id: UserId::parse(&user_id).map_err(UnexpectedError::from)?,
            },
            EventDto::EmailValidated { at, user_id } => Event::EmailValidated {
                at,
                user_id: UserId::parse(&user_id).map_err(UnexpectedError::from)?,
            },
            EventDto::LoggedIn { at, user_id } => Event::LoggedIn {
                at,
                user_id: UserId::parse(&user_id).map_err(UnexpectedError::from)?,
            },
        })
    }
}

impl From<&Event> for EventDto {
    fn from(event: &Event) -> Self {
        match event {
            Event::Registered {
                at,
                credentials,
                user_id,
            } => match credentials {
                Credentials::EmailPassword {
                    email,
                    password_hash,
                } => EventDto::Registered {
                    at: *at,
                    email: email.to_string(),
                    password_hash: password_hash.0,
                    user_id: user_id.to_string(),
                },
            },
            Event::EmailValidated { at, user_id } => EventDto::EmailValidated {
                at: *at,
                user_id: user_id.to_string(),
            },
            Event::LoggedIn { at, user_id } => EventDto::LoggedIn {
                at: *at,
                user_id: user_id.to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct UserDto {
    email: String,
    user_id: String,
}

impl From<UserDto> for User {
    fn from(dto: UserDto) -> Self {
        Self {
            email: dto.email,
            user_id: dto.user_id,
        }
    }
}

impl From<&User> for UserDto {
    fn from(projection: &User) -> Self {
        Self {
            email: projection.email.clone(),
            user_id: projection.user_id.clone(),
        }
    }
}
