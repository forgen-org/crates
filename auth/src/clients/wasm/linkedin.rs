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
    pub fn sign_in(&self) -> SignInUrl {
        SignInUrl::new(&self.client_id, &self.redirect_uri)
    }
}

#[wasm_bindgen]
pub struct SignInUrl {
    client_id: String,
    redirect_uri: String,
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
    pub fn new(client_id: &str, redirect_uri: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            redirect_uri: redirect_uri.to_string(),
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

        format!("https://www.linkedin.com/oauth/v2/authorization?response_type={}&client_id={}&redirect_uri={}&scope={}", &response_type, &self.client_id, urlencoding::encode(&self.redirect_uri), urlencoding::encode(&scopes))
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
