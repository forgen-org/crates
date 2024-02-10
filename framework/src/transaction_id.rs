#[derive(Clone, PartialEq)]
pub struct TransactionId(String);

impl TransactionId {
    pub fn default() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}
