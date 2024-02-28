use crate::application::port::*;
use forgen::*;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::Serialize;
use web_sys::window;

#[derive(Default)]
pub struct WebSys;

#[async_trait(?Send)]
impl WebView for WebSys
where
    Self: Sized,
{
    fn get_query_param(&self, key: &str) -> Option<String> {
        let current_location = window()?.document()?.location()?.href().ok()?;
        let url = web_sys::Url::new(&current_location).unwrap();
        let params = url.search_params();
        params.get(key).map(|value| value.to_string())
    }

    async fn open(&self, url: &str) -> Result<(), UnexpectedError> {
        web_sys::window()
            .ok_or(UnexpectedError::from("window not found"))?
            .open_with_url(&url)
            .map_err(|_| UnexpectedError::from("failed to open url"))?
            .ok_or(UnexpectedError::from("failed to open url"))
            .map(|_| ())
    }

    async fn post<T>(&self, url: &str, data: T) -> Result<Jwt, UnexpectedError>
    where
        T: Serialize,
    {
        let response = Request::post(url)
            .json(&data)
            .map_err(UnexpectedError::from)?
            .send()
            .await
            .map_err(UnexpectedError::from)?;

        let jwt = response.json().await.map_err(UnexpectedError::from)?;

        Ok(jwt)
    }
}

#[async_trait]
impl JwtStore for WebSys {
    async fn set(&self, jwt: &Jwt) {
        LocalStorage::set("jwt", jwt).unwrap();
    }

    async fn get(&self) -> Option<Jwt> {
        LocalStorage::get("jwt").unwrap()
    }
}
