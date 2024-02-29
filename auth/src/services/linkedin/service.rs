use super::dto::*;
use crate::application::{
    port::{LinkedInApi, LinkedInTokens},
    view::LinkedInOAuthUrl,
};
use crate::domain::scalar::Email;
use forgen::*;
use reqwest::Client;
use std::collections::HashMap;

pub struct LinkedInConfig {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub redirect_uri: String,
}

pub struct LinkedInService {
    client: Client,
    config: LinkedInConfig,
}

impl LinkedInService {
    pub fn new(config: LinkedInConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

#[service]
impl LinkedInApi for LinkedInService {
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

    async fn get_oauth_url(&self) -> Result<LinkedInOAuthUrl, UnexpectedError> {
        Ok(LinkedInOAuthUrl(format!("https://www.linkedin.com/oauth/v2/authorization?response_type={}&client_id={}&redirect_uri={}&scope={}", &"code", &self.config.client_id, &self.config.redirect_uri, "openid%20profile%20email")))
    }

    async fn sign_in(&self, code: &str) -> Result<LinkedInTokens, UnexpectedError> {
        let client_secret = self
            .config
            .client_secret
            .as_ref()
            .expect("Missing AUTH_LINKEDIN_CLIENT_SECRET");

        let mut params = HashMap::new();
        params.insert("grant_type", "authorization_code");
        params.insert("code", code);
        params.insert("client_id", &self.config.client_id);
        params.insert("client_secret", &client_secret);
        params.insert("redirect_uri", &self.config.redirect_uri);

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
}
