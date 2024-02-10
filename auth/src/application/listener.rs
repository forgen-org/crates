use super::port::*;
use super::projection::User;
use super::transaction::Transaction;
use crate::domain::scalar::*;
use crate::domain::Event;
use framework::*;
use futures::stream::StreamExt;
use std::collections::HashMap;

pub struct RecomputeUserProjections(pub Vec<Event>);

impl RecomputeUserProjections {
    async fn reflect<R>(&self, runtime: &R) -> Result<Vec<UserId>, UnexpectedError>
    where
        R: EventStore + TransactionBus + UserRepository,
        R: Send + Sync,
    {
        // Caching projections
        let mut users = HashMap::<UserId, User>::new();

        // Applying events
        for event in self.0.iter() {
            let user_id = match event {
                Event::Registered { user_id, .. } => user_id.clone(),
                Event::LoggedIn { user_id, .. } => user_id.clone(),
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

            user.apply(&event);
        }

        // Save projections
        for user in users.values() {
            UserRepository::save(runtime, user).await?;
        }

        Ok(users.keys().cloned().collect())
    }
}

#[async_trait]
impl<R> Listen<R> for RecomputeUserProjections
where
    R: EventBus + EventStore + TransactionBus + UserRepository,
    R: Send + Sync,
{
    async fn listen(runtime: &R) {
        while let Some((id, events)) = EventBus::subscribe(runtime).next().await {
            match RecomputeUserProjections(events).reflect(runtime).await {
                Ok(user_ids) => {
                    for user_id in user_ids {
                        TransactionBus::publish(
                            runtime,
                            Transaction::UserProjected {
                                id: id.clone(),
                                user_id,
                            },
                        );
                    }
                }
                Err(err) => {
                    error!("Failed to recompute user projections: {}", err);
                }
            }
        }
    }
}
