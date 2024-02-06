use super::error::ApiError;
use crate::application::auth_command::{AuthCommandError, LoginCommand, RegisterCommand};
use crate::application::auth_port::*;
use crate::application::auth_query::{GetJwtByEmail, GetJwtByEmailError};
use axum::middleware::{self, FromFnLayer};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use framework::*;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{Layer, Service};

// struct AuthLayer<R> {
//     runtime: Arc<R>,
// }

// impl<R, S> Layer<S> for AuthLayer<R> {
//     type Service = AuthService<R, S>;

//     fn layer(&self, inner: S) -> Self::Service {
//         AuthService {
//             inner,
//             runtime: self.runtime.clone(),
//         }
//     }
// }

// #[derive(Clone)]
// struct AuthService<R, S> {
//     inner: S,
//     runtime: Arc<R>,
// }

// impl<R, S, B> Service<Request<B>> for AuthService<R, S>
// where
//     R: JwtPort,
//     S: Service<Request<B>>,
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future = S::Future;

//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.inner.poll_ready(cx)
//     }

//     fn call(&mut self, req: Request<B>) -> Self::Future {
//         let token = req
//             .headers()
//             .get(header::AUTHORIZATION)
//             .and_then(|auth_header| auth_header.to_str().ok())
//             .and_then(|auth_value| {
//                 if auth_value.starts_with("Bearer ") {
//                     Some(auth_value[7..].to_owned())
//                 } else {
//                     None
//                 }
//             })
//             .map(|s| Jwt(s))
//             .unwrap(); // TODO: handle error

//         let user =
//             JwtPort::verify(self.runtime.as_ref(), &token).map_err(|_| ApiError::InvalidJwt)?;

//         req.extensions_mut().insert(user);

//         self.inner.call(req)
//     }
// }

// pub struct AuthLayer;

// impl AuthLayer {
//     pub fn new<F, R, T>(runtime: Arc<R>) -> FromFnLayer<F, R, T>
//     where
//         R: JwtPort,
//     {
//         middleware::from_fn_with_state(runtime, auth_middleware)
//     }
// }

pub async fn auth_middleware<R>(
    State(runtime): State<Arc<R>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, ApiError>
where
    R: Runtime + JwtPort,
{
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        })
        .map(|s| Jwt(s))
        .ok_or(ApiError::InvalidJwt)?;

    let user = JwtPort::verify(runtime.as_ref(), &token).map_err(|_| ApiError::InvalidJwt)?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
