use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct AuthApi {
    linkedin: LinkedInApi,
}

#[wasm_bindgen]
struct LinkedInApi;
