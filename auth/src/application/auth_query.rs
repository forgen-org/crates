// use crate::application::auth_port::*;
// use crate::application::auth_projection::User;
// use crate::domain::auth_scalar::Credentials;
// use framework::*;

// pub struct LogIn(Credentials);

// #[async_trait]
// impl<R> Query<R, User, LogInError> for LogIn
// where
//     R: Runtime + UserRepository,
// {
//     async fn execute(self, r: &R) -> Result<User, LogInError> {
//         let credentials = self.0;
//         let user =
//             UserRepository::find_one(r, UserRepositoryFilter::ByCredentials(credentials)).await?;
//         Ok(user)
//     }
// }

// #[derive(Debug, Error)]
// pub enum LogInError {
//     #[error(transparent)]
//     UserRepositoryError(#[from] UserRepositoryError),
// }
