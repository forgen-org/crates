use super::port::*;
use crate::domain::scalar::*;
use framework::*;

pub struct GetJwtByEmail {
    pub email: Email,
}

#[async_trait]
impl<R> Fetch<R> for GetJwtByEmail
where
    R: JwtPort + UserRepository,
    R: Send + Sync,
{
    type Output = Jwt;
    type Error = GetJwtByEmailError;

    async fn fetch(&self, runtime: &R) -> Result<Jwt, GetJwtByEmailError> {
        let user = UserRepository::find_by_email(runtime, &self.email)
            .await?
            .ok_or(GetJwtByEmailError::UserNotFound)?;
        let jwt = JwtPort::sign(runtime, &user)?;
        Ok(jwt)
    }
}

#[derive(Error, Debug)]
pub enum GetJwtByEmailError {
    #[error("User not found")]
    UserNotFound,

    #[error(transparent)]
    UnexpectedError(#[from] UnexpectedError),
}
