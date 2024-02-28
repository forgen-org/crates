use crate::application::port::*;
use forgen::*;
use web_sys::window;

#[derive(Default)]
pub struct WebSys;

#[async_trait]
impl WebView for WebSys {
    fn get_query_param(&self, key: &str) -> Option<String> {
        let current_location = window()?.document()?.location()?.href().ok()?;
        let url = web_sys::Url::new(&current_location).unwrap();
        let params = url.search_params();
        params.get(key).map(|value| value.to_string())
    }

    async fn push(&self, url: &str) -> Result<(), UnexpectedError> {
        web_sys::window()
            .ok_or(UnexpectedError::from("window not found"))?
            .open_with_url(&url)
            .map_err(|_| UnexpectedError::from("failed to open url"))?
            .ok_or(UnexpectedError::from("failed to open url"))
            .map(|_| ())
    }
}
