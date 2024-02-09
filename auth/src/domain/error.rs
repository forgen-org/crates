use framework::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Already registered")]
    AlreadyRegistered,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Not registered")]
    NotRegistered,
}
