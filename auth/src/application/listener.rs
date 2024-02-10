use self::event::Event;

use super::port::*;
use super::projection::User;
use crate::domain::scalar::*;
use crate::*;
use framework::*;
use futures::stream::StreamExt;
use std::collections::HashMap;

pub struct RecomputeUserProjections(pub Vec<Event>);

#[async_trait]
impl<R> Execute<R> for RecomputeUserProjections
where
    R: EventStore + UserRepository,
    R: Send + Sync,
{
    type Error = UnexpectedError;

    async fn execute(&self, runtime: &R) -> Result<(), UnexpectedError> {
        // Caching projections
        let mut users = HashMap::<UserId, User>::new();

        // Applying events
        for event in self.0.iter() {
            let user_id = match event {
                domain::Event::Registered { user_id, .. } => user_id.clone(),
                domain::Event::LoggedIn { user_id, .. } => user_id.clone(),
                _ => continue,
            };

            let user = match users.get_mut(&user_id) {
                Some(user) => user,
                None => match UserRepository::find_by_user_id(runtime, &user_id).await? {
                    Some(user) => {
                        users.insert(user_id.clone(), user);
                        users.get_mut(&user_id).unwrap()
                    }
                    None => {
                        let events = EventStore::pull_by_user_id(runtime, &user_id).await?;
                        let user = User::project(&events);
                        users.insert(user_id.clone(), user);
                        users.get_mut(&user_id).unwrap()
                    }
                },
            };

            user.apply(event);
        }

        // Save projections
        for user in users.values() {
            UserRepository::save(runtime, user).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl<R> Listen<R> for RecomputeUserProjections
where
    R: EventBus + EventStore + UserRepository,
    R: Send + Sync,
{
    async fn listen(runtime: &R) {
        while let Some(events) = EventBus::subscribe(runtime).next().await {
            if let Err(err) = RecomputeUserProjections(events).execute(runtime).await {
                error!("Failed to recompute user projections: {}", err);
            }
        }
    }
}
