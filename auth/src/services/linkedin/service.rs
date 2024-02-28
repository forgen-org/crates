use super::dto::*;
use crate::application::port::{LinkedInPort, LinkedInTokens};
use crate::domain::scalar::Email;
use forgen::*;
use reqwest::Client;
use std::collections::HashMap;
use std::env;

pub struct LinkedInService {
    client_id: String,
    client_secret: String,
    client: Client,
    redirect_uri: String,
}

impl Default for LinkedInService {
    fn default() -> Self {
        Self {
            client_id: env::var("AUTH_LINKEDIN_CLIENT_ID")
                .expect("Missing AUTH_LINKEDIN_CLIENT_ID"),
            client_secret: env::var("AUTH_LINKEDIN_CLIENT_SECRET")
                .expect("Missing AUTH_LINKEDIN_CLIENT_SECRET"),
            client: Client::new(),
            redirect_uri: env::var("AUTH_LINKEDIN_REDIRECT_URI")
                .expect("Missing AUTH_LINKEDIN_REDIRECT_URI"),
        }
    }
}

#[async_trait]
impl LinkedInPort for LinkedInService {
    async fn sign_in(&self, code: &str) -> Result<LinkedInTokens, UnexpectedError> {
        let mut params = HashMap::new();
        params.insert("grant_type", "authorization_code");
        params.insert("code", code);
        params.insert("client_id", &self.client_id);
        params.insert("client_secret", &self.client_secret);
        params.insert("redirect_uri", &self.redirect_uri);

        // Serialize your parameters into `application/x-www-form-urlencoded` format
        let body = serde_urlencoded::to_string(&params).unwrap();

        let response = self
            .client
            .post("https://www.linkedin.com/oauth/v2/accessToken")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .map_err(UnexpectedError::from)?;

        let dto = response
            .json::<ResultDto<AccessTokenDto>>()
            .await
            .map_err(UnexpectedError::from)?
            .result()?;

        Ok(LinkedInTokens::from(dto))
    }

    async fn get_email(&self, tokens: &LinkedInTokens) -> Result<Email, UnexpectedError> {
        let response = self
            .client
            .get("https://api.linkedin.com/v2/userinfo")
            .header("Authorization", format!("Bearer {}", tokens.access_token))
            .send()
            .await
            .map_err(UnexpectedError::from)?;

        let dto = response
            .json::<ResultDto<UserInfoDto>>()
            .await
            .map_err(UnexpectedError::from)?
            .result()?;

        Ok(Email::try_from(dto).map_err(UnexpectedError::from)?)
    }
}
