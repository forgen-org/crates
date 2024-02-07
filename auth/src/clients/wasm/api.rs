use wasm_bindgen::prelude::*;

use super::linkedin;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct AuthApi {
    linkedin: Option<linkedin::Api>,
}

#[wasm_bindgen]
impl AuthApi {
    #[wasm_bindgen(constructor)]
    pub fn new(linkedin: Option<linkedin::Api>) -> Self {
        Self { linkedin }
    }

    #[wasm_bindgen]
    pub fn linkedin(self) -> linkedin::Api {
        self.linkedin.expect("LinkedIn API is not available")
    }
}
