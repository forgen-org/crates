use super::common;
use gloo_net::http::Request;
use std::env;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Api {
    auth_uri: String,
    client_id: String,

    #[wasm_bindgen(skip)]
    common: Arc<common::Api>,
}

impl Api {
    pub fn new(common: Arc<common::Api>) -> Self {
        let origin = web_sys::window().unwrap().location().origin().unwrap();

        Self {
            auth_uri: format!("{}/api/auth/linkedin", origin),
            client_id: env::var("AUTH_LINKEDIN_CLIENT_ID")
                .expect("Missing AUTH_LINKEDIN_CLIENT_ID"),
            common,
        }
    }
}

#[wasm_bindgen]
impl Api {
    #[wasm_bindgen]
    pub async fn listen_for_code(&self) {
        let current_location = web_sys::window().unwrap().location().href().unwrap();
        let url = web_sys::Url::new(&current_location).unwrap();
        let params = url.search_params();

        if let Some(code) = params.get("code") {
            let response = Request::get(&format!("{}?code={}", &self.auth_uri, code))
                .build()
                .unwrap()
                .send()
                .await
                .unwrap();
            self.common
                .set_access_token(&response.text().await.unwrap());
        }
    }

    #[wasm_bindgen]
    pub fn set_auth_uri(&mut self, auth_uri: &str) {
        self.auth_uri = auth_uri.to_string();
    }

    #[wasm_bindgen]
    pub fn sign_in(&self) -> SignInUrl {
        SignInUrl::new(&self.client_id)
    }
}

#[wasm_bindgen]
pub struct SignInUrl {
    client_id: String,
    response_type: Responsetype,
    scopes: Vec<Scope>,
}

#[wasm_bindgen]
pub enum Responsetype {
    Code,
}

#[wasm_bindgen]
pub enum Scope {
    Openid,
    Profile,
    Email,
}

#[wasm_bindgen]
impl SignInUrl {
    #[wasm_bindgen(constructor)]
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            response_type: Responsetype::Code,
            scopes: vec![Scope::Openid, Scope::Profile, Scope::Email],
        }
    }

    #[wasm_bindgen]
    pub fn add_scope(mut self, scope: Scope) -> Self {
        self.scopes.push(scope);
        self
    }

    #[wasm_bindgen]
    pub fn response_type(mut self, response_type: Responsetype) -> Self {
        self.response_type = response_type;
        self
    }

    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        let response_type = match self.response_type {
            Responsetype::Code => "code",
        };
        let scopes = self
            .scopes
            .iter()
            .map(|scope| match scope {
                Scope::Openid => "openid",
                Scope::Profile => "profile",
                Scope::Email => "email",
            })
            .collect::<Vec<&str>>()
            .join(" ");

        let current_location = web_sys::window().unwrap().location().href().unwrap();

        format!("https://www.linkedin.com/oauth/v2/authorization?response_type={}&client_id={}&redirect_uri={}&scope={}", &response_type, &self.client_id, urlencoding::encode(&current_location), urlencoding::encode(&scopes))
    }

    #[wasm_bindgen]
    pub fn open(&self) {
        let url = self.to_string();
        web_sys::window()
            .unwrap()
            .open_with_url(&url)
            .unwrap()
            .unwrap();
    }
}
