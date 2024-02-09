use framework::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Already registered")]
    AlreadyRegistered,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Not registered")]
    NotRegistered,
}
