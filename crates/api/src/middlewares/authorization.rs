use application::services::{Claims, IJwtService, JwtService};
use axum::{
    body::Body, extract::State, http::{Request, StatusCode}, middleware::Next, response::{IntoResponse, Response}
};
use domain::models::users::enums::user_role_type::UserRoleType;
use jsonwebtoken::{DecodingKey, Validation, decode};

pub async fn auth(mut req: Request<Body>, next: Next) -> Response {
    let auth_header = match req.headers().get(axum::http::header::AUTHORIZATION) {
        Some(h) => match h.to_str() {
            Ok(v) => v,
            Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
        },
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let token = match auth_header.strip_prefix("Bearer ") {
        Some(t) => t,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let secret = JwtService::get_secret();
    let decoded 
    = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(data) => data.claims,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    req.extensions_mut().insert(decoded);

    next.run(req).await
}

pub async fn require_roles(
    State(allowed_roles): State<Vec<UserRoleType>>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let claims = match req.extensions().get::<Claims>() {
        Some(claims) => claims,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if !allowed_roles.contains(&claims.role) {
        return StatusCode::FORBIDDEN.into_response();
    }

    next.run(req).await
}