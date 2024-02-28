use super::runtime::SignalSub;
use crate::{
    port::SignalPub,
    scalar::{TransactionId, UserId},
    signal::Signal,
};

pub async fn wait_for_user<R>(runtime: &R, transaction_id: Option<TransactionId>) -> UserId
where
    R: SignalPub + SignalSub,
{
    let mut receiver = SignalSub::subscribe(runtime);

    loop {
        match receiver.recv().await.unwrap() {
            Signal::UserProjected {
                user_id,
                transaction_id: received_transaction_id,
            } => {
                if transaction_id.is_none() || received_transaction_id == transaction_id {
                    return user_id;
                }
            }
            _ => {}
        }
    }
}
