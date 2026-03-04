use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

const X_FORWARDED_ACCESS_TOKEN: &str = "X-Forwarded-Access-Token";
const X_FORWARDED_USER: &str = "X-Forwarded-User";
const X_FORWARDED_EMAIL: &str = "X-Forwarded-Email";
const DANGER_JWT_OVERRIDE: &str = "DANGER_JWT_OVERRIDE";

#[derive(Serialize, Deserialize, Debug)]
pub struct Jwt {
    pub sub: String,
    pub email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
}

impl<S> FromRequestParts<S> for Jwt
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1) token override (env) or forwarded access token header
        if let Some(token) = access_token(parts)
            && let Some(jwt) = decode_jwt_payload(&token)
        {
            return Ok(jwt);
        }

        // 2) fallback: forwarded user/email headers
        if let Some(jwt) = forwarded_identity(parts) {
            return Ok(jwt);
        }

        Err((
            StatusCode::UNAUTHORIZED,
            "Didn't find an authentication header",
        ))
    }
}

fn header_str<'a>(parts: &'a Parts, name: &str) -> Option<&'a str> {
    parts.headers.get(name).and_then(|h| h.to_str().ok())
}

fn access_token(parts: &Parts) -> Option<String> {
    std::env::var(DANGER_JWT_OVERRIDE)
        .ok()
        .or_else(|| header_str(parts, X_FORWARDED_ACCESS_TOKEN).map(str::to_owned))
}

fn forwarded_identity(parts: &Parts) -> Option<Jwt> {
    let sub = header_str(parts, X_FORWARDED_USER)?;
    let email = header_str(parts, X_FORWARDED_EMAIL)?;
    Some(Jwt {
        sub: sub.to_owned(),
        email: email.to_owned(),
        given_name: None,
        family_name: None,
    })
}

fn decode_jwt_payload(token: &str) -> Option<Jwt> {
    // JWT is "header.payload.signature"
    let payload_b64 = token.split('.').nth(1)?;
    let payload = URL_SAFE_NO_PAD.decode(payload_b64).ok()?;
    serde_json::from_slice::<Jwt>(&payload).ok()
}
