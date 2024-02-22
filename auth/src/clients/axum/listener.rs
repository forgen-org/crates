use crate::{
    port::SignalBus,
    scalar::{TransactionId, UserId},
    signal::Signal,
};
use forgen::*;
use tokio::sync::oneshot;

pub async fn wait_for_user<R>(runtime: &R, transaction_id: Option<TransactionId>) -> UserId
where
    R: SignalBus,
{
    let (tx, rx) = oneshot::channel::<UserId>();
    let tx = std::sync::Mutex::new(Some(tx)); // Make `tx` safely shareable and mutable across threads

    runtime.subscribe(move |signal| {
        if let Signal::UserProjected(metadata) = signal {
            info!("UserProjected {:?}", metadata);
            if let Some(user_id) = metadata.user_id {
                if transaction_id.is_none() || metadata.transaction_id == transaction_id {
                    // Take the sender out of the `Option`, leaving `None` in its place
                    if let Some(sender) = tx.lock().unwrap().take() {
                        let _ = sender.send(user_id);
                    }
                }
            }
        }
    });

    rx.await.unwrap()
}
