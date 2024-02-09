use super::{
    event::Credentials,
    scalar::{Email, Password},
};

pub enum Message {
    Register { method: RegisterMethod },
    LogIn { method: RegisterMethod },
}

pub enum RegisterMethod {
    EmailPassword { email: Email, password: Password },
}

impl From<&RegisterMethod> for Credentials {
    fn from(method: &RegisterMethod) -> Self {
        match method {
            RegisterMethod::EmailPassword { email, password } => Credentials::EmailPassword {
                email: email.clone(),
                password_hash: password.into(),
            },
        }
    }
}
