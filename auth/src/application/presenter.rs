use super::command::ConnectLinkedIn;
use super::port::*;
use forgen::*;

#[derive(Clone, Default, PartialEq)]
pub struct LoginForm {
    email: String,
    error: Option<String>,
    password: String,
}

pub enum LoginAction {
    Init {
        linkedin_api_url: Option<String>,
    },
    LoginWithLinkedIn {
        client_id: String,
        redirect_uri: String,
    },
}

#[async_trait(?Send)]
impl<R> Reduce<R> for LoginForm
where
    R: JwtStore + WebView,
    R: Send + Sync,
{
    type Action = LoginAction;

    async fn reduce(&self, runtime: &R, action: Self::Action) -> Self {
        let mut state = self.clone();

        match action {
            Self::Action::Init { linkedin_api_url } => {
                if let Some(linkedin_api_url) = linkedin_api_url {
                    if let Some(code) = WebView::get_query_param(runtime, "code") {
                        if let Ok(jwt) = WebView::post(
                            runtime,
                            &linkedin_api_url,
                            ConnectLinkedIn {
                                code,
                                transaction_id: None,
                            },
                        )
                        .await
                        {
                            info!("jwt: {}", &jwt.0);
                            JwtStore::set(runtime, &jwt).await;
                        }
                    }
                }
            }
            Self::Action::LoginWithLinkedIn {
                client_id,
                redirect_uri,
            } => {
                let url = format!("https://www.linkedin.com/oauth/v2/authorization?response_type={}&client_id={}&redirect_uri={}&scope={}", &"code", &client_id, &redirect_uri, "openid%20profile%20email");

                if let Err(error) = WebView::open(runtime, &url).await {
                    state.error = Some(error.to_string());
                }
            }
        }

        state
    }
}
