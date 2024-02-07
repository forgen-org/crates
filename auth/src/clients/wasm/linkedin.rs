use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Api {
    client_id: String,
    redirect_uri: String,
}

#[wasm_bindgen]
impl Api {
    #[wasm_bindgen(constructor)]
    pub fn new(client_id: &str, redirect_uri: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            redirect_uri: redirect_uri.to_string(),
        }
    }

    #[wasm_bindgen]
    pub fn sign_in(&self) -> String {
        self.sign_in_with_params(SignInParams::new())
    }

    #[wasm_bindgen]
    pub fn sign_in_with_params(&self, params: SignInParams) -> String {
        let scopes = params
            .scopes
            .iter()
            .map(|scope| scope.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        format!("https://www.linkedin.com/oauth/v2/authorization?response_type={}&client_id={}&redirect_uri={}&scope={}", &params.response_type, &self.client_id, urlencoding::encode(&self.redirect_uri), urlencoding::encode(&scopes))
    }
}

#[wasm_bindgen]
pub struct SignInParams {
    response_type: Responsetype,
    scopes: Vec<Scope>,
}

#[wasm_bindgen]
impl SignInParams {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            response_type: Responsetype::Code,
            scopes: vec![Scope::Openid, Scope::Profile, Scope::Email],
        }
    }
}

#[wasm_bindgen]
pub enum Responsetype {
    Code,
}

impl std::fmt::Display for Responsetype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Responsetype::Code => write!(f, "code"),
        }
    }
}

#[wasm_bindgen]
pub enum Scope {
    Openid,
    Profile,
    Email,
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Scope::Openid => write!(f, "openid"),
            Scope::Profile => write!(f, "profile"),
            Scope::Email => write!(f, "email"),
        }
    }
}
