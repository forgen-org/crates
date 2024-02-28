use crate::application::port::{Jwt, JwtPort};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

pub async fn jwt_guard<R>(
    State(runtime): State<Arc<R>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, Response>
where
    R: JwtPort,
    R: Send + Sync,
{
    match req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(value[7..].to_owned())
            } else {
                None
            }
        })
        .map(|s| Jwt(s))
    {
        Some(token) => {
            if let Ok(user) = JwtPort::verify(runtime.as_ref(), &token).await {
                req.extensions_mut().insert(Arc::new(user));
                Ok(next.run(req).await)
            } else {
                Err((StatusCode::UNAUTHORIZED, "Invalid JWT").into_response())
            }
        }
        None => Err((StatusCode::UNAUTHORIZED, "No JWT").into_response()),
    }
}
