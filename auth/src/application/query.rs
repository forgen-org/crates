use super::port::*;
use crate::domain::scalar::*;
use forgen::*;

pub struct GetJwtByUserId {
    pub user_id: UserId,
}

#[async_trait]
impl<R> Querier<R> for GetJwtByUserId
where
    R: JwtPort + UserRepository,
    R: Send + Sync,
{
    type Output = Jwt;
    type Error = GetJwtByEmailError;

    async fn fetch(&self, runtime: &R) -> Result<Jwt, GetJwtByEmailError> {
        let user = UserRepository::find_by_user_id(runtime, &self.user_id)
            .await?
            .ok_or(GetJwtByEmailError::UserNotFound)?;
        let jwt = JwtPort::sign(runtime, &user).await?;
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
