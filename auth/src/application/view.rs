use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedInOAuthUrl(pub String);
