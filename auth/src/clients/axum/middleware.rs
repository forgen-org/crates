use super::error::ApiError;
use crate::application::auth_port::*;
use axum::{
    body::Body,
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use framework::*;
use std::sync::Arc;

pub async fn auth_middleware<R>(
    State(runtime): State<Arc<R>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, ApiError>
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
