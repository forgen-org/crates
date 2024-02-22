// use super::port::*;
// use super::projection::User;
// use super::scalar::*;
// use crate::domain;
// use forgen::*;

// #[derive(Default)]
// pub struct TestRuntime {}

// impl EventStore for TestRuntime {
//     fn identify_by_email(&self, _email: &Email) -> Result<Option<UserId>, UnexpectedError> {
//         Ok(Some(UserId::default()))
//     }

//     fn pull_by_user_id(&self, _user_id: &UserId) -> Result<Vec<domain::Event>, UnexpectedError> {
//         Ok(vec![])
//     }

//     fn push(&self, _events: &[domain::Event]) -> Result<(), UnexpectedError> {
//         Ok(())
//     }
// }

// impl UserRepository for TestRuntime {
//     fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UnexpectedError> {
//         Ok(Some(User {
//             email: "email@example.com".to_string(),
//             user_id: user_id.to_string(),
//         }))
//     }

//     fn save(&self, _projection: &User) -> Result<(), UnexpectedError> {
//         Ok(())
//     }
// }
// impl JwtPort for TestRuntime {
//     fn sign(&self, _user: &User) -> Result<Jwt, UnexpectedError> {
//         Ok(Jwt("jwt".to_string()))
//     }

//     fn verify(&self, _token: &Jwt) -> Result<User, UnexpectedError> {
//         Ok(User {
//             email: "email@example.com".to_string(),
//             user_id: UserId::default().to_string(),
//         })
//     }
// }
