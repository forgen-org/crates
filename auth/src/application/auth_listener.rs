use super::auth_port::*;
use crate::domain::auth_message::{AuthError, AuthMessage};
use framework::*;

pub struct AuthListener {
    pub user_id: UserId,
    pub message: AuthMessage,
}

#[async_trait]
impl<R> Command<R, AuthListenerError> for AuthListener
where
    R: Runtime + AuthStore + UserRepository,
{
    async fn execute(self, runtime: &R) -> Result<(), AuthListenerError> {
        // Pull existing events
        let existing_events = AuthStore::pull(runtime, &self.user_id).await?;

        // Send the message
        let new_events = self.message.send(&existing_events)?;

        // Push new events
        AuthStore::push(runtime, &self.user_id, &new_events).await?;

        // Recompute projections
        let mut projection = UserRepository::find_one(runtime, &self.user_id).await?;
        projection.apply(&new_events);
        UserRepository::save(runtime, &self.user_id, &projection).await?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum AuthListenerError {
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    AuthStoreError(#[from] AuthStoreError),
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
}
