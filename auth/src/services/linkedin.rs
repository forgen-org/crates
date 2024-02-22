// use crate::application::{Email, LinkedInPort, LinkedInTokens};
// use forgen::*;
// use reqwest::Client;
// use serde::Deserialize;
// use std::collections::HashMap;

// struct LinkedInService {
//     client: Client,
//     client_id: String,
//     client_secret: Option<String>,
//     redirect_uri: String,
// }

//
// impl LinkedInPort for LinkedInService {
//     fn sign_in(&self, code: &str) -> Result<LinkedInTokens, UnexpectedError> {
//         let client_secret = self
//             .client_secret
//             .as_ref()
//             .ok_or(UnexpectedError::from("Client secret is missing"))?;

//         let mut params = HashMap::new();
//         params.insert("grant_type", "authorization_code");
//         params.insert("code", code);
//         params.insert("client_id", &self.client_id);
//         params.insert("client_secret", &client_secret);
//         params.insert("redirect_uri", &self.redirect_uri);

//         // Serialize your parameters into `application/x-www-form-urlencoded` format
//         let body = serde_urlencoded::to_string(&params).unwrap();

//         self.client
//             .post("https://www.linkedin.com/oauth/v2/accessToken")
//             .header("Content-Type", "application/x-www-form-urlencoded")
//             .body(body)
//             .send()
//
//             .unwrap()
//             .json::<TokensResponse>()
//
//             .map(LinkedInTokens::from)
//             .map_err(UnexpectedError::from)
//     }

//     fn get_email(&self, tokens: &LinkedInTokens) -> Result<Email, UnexpectedError> {
//         self
//             .client
//             .get("https://api.linkedin.com/v2/emailAddress?q=members&projection=(elements*(handle~))")
//             .header("Authorization", format!("Bearer {}", tokens.access_token))
//             .send()
//
//             .unwrap()
//             .json::<EmailResponse>()
//
//             .map_err(UnexpectedError::from)
//             .and_then(Email::try_from)
//     }
// }

// #[derive(Deserialize)]
// struct TokensResponse {
//     access_token: String,
//     // expires_in: u64,
//     refresh_token: String,
//     // refresh_token_expires_in: u64,
//     // scope: String,
// }

// impl From<TokensResponse> for LinkedInTokens {
//     fn from(response: TokensResponse) -> Self {
//         LinkedInTokens {
//             access_token: response.access_token,
//             refresh_token: response.refresh_token,
//         }
//     }
// }

// #[derive(Deserialize)]
// struct EmailResponse {
//     elements: Vec<EmailElement>,
// }

// #[derive(Deserialize)]
// struct EmailElement {
//     handle: EmailHandle,
// }

// #[derive(Deserialize)]
// struct EmailHandle {
//     email_address: String,
// }

// impl TryFrom<EmailResponse> for Email {
//     type Error = UnexpectedError;

//     fn try_from(value: EmailResponse) -> Result<Self, Self::Error> {
//         Email::parse(&value.elements[0].handle.email_address).map_err(UnexpectedError::from)
//     }
// }
