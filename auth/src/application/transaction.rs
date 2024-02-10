use crate::domain::scalar::UserId;
use framework::TransactionId;

#[derive(Clone, PartialEq)]
pub enum Transaction {
    UserProjected { id: TransactionId, user_id: UserId },
}
