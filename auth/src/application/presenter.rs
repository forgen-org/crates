use super::port::*;
use forgen::*;

#[derive(Default, Clone, PartialEq)]
pub struct LoginForm {
    email: String,
    error: Option<String>,
    password: String,
}

pub enum LoginAction {
    Init,
    #[cfg(feature = "linkedin")]
    LoginWithLinkedIn {
        client_id: String,
        redirect_uri: String,
    },
}

#[async_trait]
impl<R> Reduce<R> for LoginForm
where
    R: WebView,
    R: Send + Sync,
{
    type Action = LoginAction;

    async fn reduce(&self, runtime: &R, action: Self::Action) -> Self {
        let mut state = self.clone();

        match action {
            Self::Action::Init => {
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
            #[cfg(feature = "linkedin")]
            Self::Action::LoginWithLinkedIn {
                client_id,
                redirect_uri,
            } => {
                let url = format!("https://www.linkedin.com/oauth/v2/authorization?response_type={}&client_id={}&redirect_uri={}&scope={}", &"code", &client_id, &redirect_uri, "open_id,profile,email");

                if let Err(error) = WebView::push(runtime, &url).await {
                    state.error = Some(error.to_string());
                }
            }
        }

        state
    }
}
