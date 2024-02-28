use super::{common, linkedin};
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Api {
    #[wasm_bindgen(skip)]
    common: Arc<common::Api>,

    linkedin: linkedin::Api,
}

#[wasm_bindgen]
impl Api {
    #[wasm_bindgen(constructor)]
    pub fn default() -> Self {
        let common = Arc::new(common::Api::default());
        Self {
            common: common.clone(),
            linkedin: linkedin::Api::new(common.clone()),
        }
    }
}
