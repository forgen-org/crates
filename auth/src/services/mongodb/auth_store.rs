use super::event_dto::EventDto;
use super::MongoDbService;
use crate::application::event::Event;
use crate::application::port::*;
use crate::application::scalar::*;
use framework::*;
use futures::TryStreamExt;
use mongodb::bson::doc;

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
            .and_then(|events: Vec<EventDto>| {
                events
                    .into_iter()
                    .map(Event::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
    }

    async fn push(&self, events: &[Event]) -> Result<(), UnexpectedError> {
        self.event
            .insert_many(events.iter().map(EventDto::from).collect::<Vec<_>>(), None)
            .await
            .map(|_| ())
            .map_err(UnexpectedError::from)
    }
}
