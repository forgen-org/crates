use super::port::*;

pub struct AuthForm {
    email: String,

    error: Option<String>,

    #[cfg(feature = "linkedin")]
    linkedin_client_id: String,

    #[cfg(feature = "linkedin")]
    linkedin_redirect_uri: String,

    password: String,
}

impl Default for AuthForm {
    fn default() -> Self {
        Self {
            email: "".to_string(),

            error: None,

            #[cfg(feature = "linkedin")]
            linkedin_client_id: std::env::var("AUTH_LINKEDIN_CLIENT_ID")
                .expect("Missing AUTH_LINKEDIN_CLIENT_ID"),

            #[cfg(feature = "linkedin")]
            linkedin_redirect_uri: std::env::var("AUTH_LINKEDIN_REDIRECT_URI")
                .expect("Missing AUTH_LINKEDIN_REDIRECT_URI"),

            password: "".to_string(),
        }
    }
}

impl AuthForm {
    pub async fn on_mount<R>(&mut self, runtime: &R) -> ()
    where
        R: WebView,
    {
        #[cfg(feature = "linkedin")]
        {
            let code = WebView::get_query_param(runtime, "code");
            // if let Some(code) = code {
            // let command = ConnectLinkedIn {
            //     code,
            //     transaction_id: None,
            // };
            // if let Err(error) = command.execute(runtime).await {
            //     self.error = Some(error.to_string());
            // }
            // }
            panic!("code: {:?}", code);
        }
    }

    #[cfg(feature = "linkedin")]
    pub async fn login_with_linked_in<R>(&mut self, runtime: &R) -> ()
    where
        R: Observer<AuthForm> + WebView,
    {
        let url = format!("https://www.linkedin.com/oauth/v2/authorization?response_type={}&client_id={}&redirect_uri={}&scope={}", &"code", &self.linkedin_client_id, &self.linkedin_redirect_uri, "open_id,profile,email");

        if let Err(error) = WebView::push(runtime, &url).await {
            self.error = Some(error.to_string());
        }

        Observer::notify(runtime, self);
    }
}
