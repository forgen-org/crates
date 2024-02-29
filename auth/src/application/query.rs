use super::port::*;
use super::view::{Jwt, LinkedInOAuthUrl};
use crate::domain::scalar::*;
use forgen::*;

pub struct GetJwtByUserId {
    pub user_id: UserId,
}

#[service]
impl<R> Fetch<R> for GetJwtByUserId
where
    R: JwtPort + UserRepository,
    R: Send + Sync,
{
    type Output = Jwt;
    type Error = GetJwtByEmailError;

    async fn fetch(&self, runtime: &R) -> Result<Self::Output, Self::Error> {
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

pub struct GetLinkedInOAuthUrl;

#[service]
impl<R> Fetch<R> for GetLinkedInOAuthUrl
where
    R: LinkedInApi,
    R: Send + Sync,
{
    type Output = LinkedInOAuthUrl;
    type Error = UnexpectedError;

    async fn fetch(&self, runtime: &R) -> Result<Self::Output, Self::Error> {
        LinkedInApi::get_oauth_url(runtime).await
    }
}
