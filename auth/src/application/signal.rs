use super::scalar::TransactionId;
use crate::domain::scalar::UserId;
use crate::domain::Event;

#[derive(Clone)]
pub enum Signal {
    EventsEmitted(Vec<Event>, Metadata),
    UserProjected(Metadata),
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub at: chrono::DateTime<chrono::Utc>,
    pub transaction_id: Option<TransactionId>,
    pub user_id: Option<UserId>,
}

impl Metadata {
    pub fn default() -> Self {
        Self {
            at: chrono::Utc::now(),
            transaction_id: None,
            user_id: None,
        }
    }

    pub fn with_transaction_id(mut self, transaction_id: TransactionId) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }

    pub fn with_user_id(mut self, user_id: UserId) -> Self {
        self.user_id = Some(user_id);
        self
    }
}
