use application::auth::{
    application::{
        command::Command,
        port::{Jwt, TransactionId},
        query::GetJwtByUserId,
    },
    runtime::Runtime,
};
use axum::{extract::State, http::StatusCode, response::Response, routing::post, Json, Router};
use forgen::*;
use listenfd::ListenFd;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
pub async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let runtime = Arc::new(Runtime::new().await);

    runtime.as_ref().tokio.init(runtime.clone());

    // let middleware = axum::middleware::from_fn_with_state(runtime.clone(), auth_middleware);

    let app = Router::new()
        .nest(
            "/api",
            Router::new().route(
                "/auth",
                post(auth_command_handler).with_state(runtime.clone()),
            ),
        )
        .fallback_service(ServeDir::new("dist").fallback(ServeFile::new("dist/index.html")));

    // run our app with hyper, listening globally on port 3000
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind("127.0.0.1:3030").await.unwrap(),
    };

    println!(
        "Server is running on http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

async fn auth_command_handler(
    State(runtime): State<Arc<Runtime>>,
    Json(command): Json<Command>,
) -> Response {
    match command {
        Command::ConnectLinkedIn(connect_linkedin) => {
            connect_linkedin.execute(runtime.as_ref()).await.unwrap();
        }
        Command::Login(login) => {
            login.execute(runtime.as_ref()).await.unwrap();
        }
        Command::Register(register) => {
            register.execute(runtime.as_ref()).await.unwrap();
        }
        _ => {}
    }
}

async fn wait_for_jwt(
    runtime: Arc<Runtime>,
    transaction_id: Option<TransactionId>,
) -> Result<Jwt, Response> {
    let mut receiver = runtime.as_ref().tokio.subscribe();

    loop {
        if let Signal::UserProjected {
            user_id,
            transaction_id: received_transaction_id,
        } = receiver.recv().await.unwrap()
        {
            {
                if transaction_id.is_none() || received_transaction_id == transaction_id {
                    let query = GetJwtByUserId { user_id };

                    let jwt = query.fetch(runtime.as_ref()).await.map_err(|err| {
                        (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)).into_response()
                    })?;

                    return Ok(jwt);
                }
            }
        }
    }
}
