pub use crate::domain::scalar::*;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TransactionId(pub Uuid);

impl Default for TransactionId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl ToString for TransactionId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
