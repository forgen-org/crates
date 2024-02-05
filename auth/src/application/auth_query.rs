use crate::application::auth_port::*;
use crate::domain::auth_scalar::EmailError;
use framework::*;

pub struct GetJwtByEmail {
    pub email: String,
}

#[async_trait]
impl<R> Query<R, Jwt, GetJwtByEmailError> for GetJwtByEmail
where
    R: Runtime + JwtPort + UserRepository,
{
    async fn execute(&self, runtime: &R) -> Result<Jwt, GetJwtByEmailError> {
        let email = Email::parse(&self.email)?;
        let user = UserRepository::find_by_email(runtime, &email)
            .await?
            .ok_or(GetJwtByEmailError::UserNotFound)?;
        let jwt = JwtPort::sign(runtime, &user).await?;
        Ok(jwt)
    }
}

#[derive(Debug, Error)]
pub enum GetJwtByEmailError {
    #[error("User not found")]
    UserNotFound,

    #[error(transparent)]
    EmailError(#[from] EmailError),
    #[error(transparent)]
    ServiceError(#[from] ServiceError),
}
