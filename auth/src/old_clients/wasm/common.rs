use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Api {}

#[wasm_bindgen]
impl Api {
    #[wasm_bindgen(constructor)]
    pub fn default() -> Self {
        Self {}
    }

    #[wasm_bindgen]
    pub fn get_access_token(&self) -> Option<String> {
        LocalStorage::get("auth_access_token").ok()
    }

    #[wasm_bindgen]
    pub fn set_access_token(&self, access_token: &str) {
        LocalStorage::set("auth_access_token", access_token).unwrap();

        gloo_console::log!("Access token: {}", access_token);
    }
}
