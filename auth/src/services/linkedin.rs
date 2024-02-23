use crate::application::port::{LinkedInPort, LinkedInTokens};
use crate::application::scalar::Email;
use forgen::*;
use reqwest::Client;
use serde::Deserialize;
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

        info!(
            "{:?}",
            self.client
                .post("https://www.linkedin.com/oauth/v2/accessToken")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body)
                .send()
                .await
                .unwrap()
                .text()
                .await
        );

        // Ok("{\"access_token\":\"AQVb_uDIhXjl9Cc8LJnQxUIqoJW2I7hkwKzwEe2LuDZRYokMtV4EUE2Kh1P8qo8gJf8cjfs4040bAtaRAqH5qtKy94aH1s7jj-iJ_nzSQ4W5z07k_XQA4hUNUuq1yBG_lDoaP1p_fdKkRnO8YlbwiVKk3AHZubbi8alBBxqlOrRzL-HxgnzkATS0YLr73wF-TjBu_Qh0iDJu5Ew5VRaBIt970p3Vew138CgnwI4cVkpNidsPtXXp6h_xXs2Av8BeZfHwB_D6TblJ8Ajw4drjZ_jCRQvxs8sTlYnO8m72fHB32zM2nERQmsJFUQU0_X_Xbb2hAALC5unNT7D5HWUjbdqp377n-Q\",\"expires_in\":5183999,\"scope\":\"email,openid,profile\",\"token_type\":\"Bearer\",\"id_token\":\"eyJ6aXAiOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImQ5Mjk2NjhhLWJhYjEtNGM2OS05NTk4LTQzNzMxNDk3MjNmZiIsImFsZyI6IlJTMjU2In0.eyJpc3MiOiJodHRwczovL3d3dy5saW5rZWRpbi5jb20iLCJhdWQiOiI3N2Jtdmxod2lwdDY1OSIsImlhdCI6MTcwODY0MTg4MSwiZXhwIjoxNzA4NjQ1NDgxLCJzdWIiOiJ4X2YwVDZ5QXFQIiwibmFtZSI6IkFsZXhhbmRyZSBIYW5vdCIsImdpdmVuX25hbWUiOiJBbGV4YW5kcmUiLCJmYW1pbHlfbmFtZSI6Ikhhbm90IiwicGljdHVyZSI6Imh0dHBzOi8vbWVkaWEubGljZG4uY29tL2Rtcy9pbWFnZS9ENEQwM0FRRmJhek96b0ZzdjF3L3Byb2ZpbGUtZGlzcGxheXBob3RvLXNocmlua18xMDBfMTAwLzAvMTcwODYzMjAzNDkwNz9lPTIxNDc0ODM2NDcmdj1iZXRhJnQ9TWlUTnVRWVhHMTRBY0M1M2gxXzJNc1E5Sk93QlA3ZE9MUDhWRjR3VWlTbyIsImVtYWlsIjoiYWxleGFuZHJlLmhhbm90QGVkaGVjLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjoidHJ1ZSIsImxvY2FsZSI6ImZyX0ZSIn0.mhhiMh_5G7ahX0McuElGq5HfTKxN6lgQYtHEEk23QoKueouuGqHVzdtq5dAFCSSg08GpQuRq0zGmrE5RrpAsc6ecP0__M7I4Z2asZpuvSkwf35-5jPgR7iFeTkzZ_epZKFR9Kf4Lc9_k4wWexoSGZ6wmno9qEMlgmO_46CTIM7pAHNBUKRf4zHTPDYVPp9lMzrXsE-yqh6hOOozgpgM_sGPDpiQuJWCfyLXwJGJBt40qiyXfIavScDRhmEsHxVkNc-xtK4kCDmeyfAtzcHW2b_ianzLKt7RnkogC5ExNzBNNdsaXS2fkoKqiqdkRSfn0jcHPyKXYumdxnwBiG_kRUIR7-7yxZATGTHQ0e0YKdTo0trxPqFA96Kwh_s9zUgLnne9GUYr1z6cqpwDseu2Gvirr1l3_A8cqU-XJfLrbrT_cGYXFzN6h3KOcNACMonG3ZKq831LSmWb82oSdHUHIpIuv3csVQeolHiBuP7H8LA2smuQymjKIwrMzjVZSE2DtYh9ZyVFmHMilQvEm92HEId2GlLh4TjIgCbHzBGERc7_X_StTzqiqqx0Q2c5rkWuYqOBu2SLNnxV7eTjkPs4bxMOnONJDBZMWpUkGIAIT7G0NYS5d1zYhzziSIDYB0MGduX9NJU8jx11PvcU0HHReNHdFe-LyrHHw90TceT4yddM\"}")

        // Ok("{\"error\":\"invalid_request\",\"error_description\":\"Unable to retrieve access token: appid/redirect uri/code verifier does not match authorization code. Or authorization code expired. Or external member binding exists\"}")

        // self.client
        //     .post("https://www.linkedin.com/oauth/v2/accessToken")
        //     .header("Content-Type", "application/x-www-form-urlencoded")
        //     .body(body)
        //     .send()
        //     .unwrap()
        //     .json::<TokensResponse>()
        //     .map(LinkedInTokens::from)
        //     .map_err(UnexpectedError::from)

        todo!()
    }

    async fn get_email(&self, tokens: &LinkedInTokens) -> Result<Email, UnexpectedError> {
        self.client
            .get("https://api.linkedin.com/v2/emailAddress?q=members&projection=(elements*(handle~))")
            .header("Authorization", format!("Bearer {}", tokens.access_token))
            .send().await
            .map_err(UnexpectedError::from)?
            .json::<EmailResponse>().await
            .map_err(UnexpectedError::from)
            .and_then(Email::try_from)
    }
}

#[derive(Deserialize)]
struct TokensResponse {
    access_token: String,
    // expires_in: u64,
    refresh_token: String,
    // refresh_token_expires_in: u64,
    // scope: String,
}

impl From<TokensResponse> for LinkedInTokens {
    fn from(response: TokensResponse) -> Self {
        LinkedInTokens {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
        }
    }
}

#[derive(Deserialize)]
struct EmailResponse {
    elements: Vec<EmailElement>,
}

#[derive(Deserialize)]
struct EmailElement {
    handle: EmailHandle,
}

#[derive(Deserialize)]
struct EmailHandle {
    email_address: String,
}

impl TryFrom<EmailResponse> for Email {
    type Error = UnexpectedError;

    fn try_from(value: EmailResponse) -> Result<Self, Self::Error> {
        Email::parse(&value.elements[0].handle.email_address).map_err(UnexpectedError::from)
    }
}
