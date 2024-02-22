use forgen::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Already registered")]
    AlreadyRegistered,

    #[error("Invalid email")]
    InvalidEmail,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Not registered")]
    NotRegistered,
}
