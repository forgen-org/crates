use super::scalar::TransactionId;
use crate::domain::scalar::UserId;
use crate::domain::Event;

#[derive(Clone)]
pub enum Signal {
    EventsEmitted {
        events: Vec<Event>,
        transaction_id: Option<TransactionId>,
        user_id: Option<UserId>,
    },
    UserProjected {
        transaction_id: Option<TransactionId>,
        user_id: UserId,
    },
}
