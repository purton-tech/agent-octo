use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const X_FORWARDED_ACCESS_TOKEN: &str = "X-Forwarded-Access-Token";
const X_FORWARDED_USER: &str = "X-Forwarded-User";
const X_FORWARDED_EMAIL: &str = "X-Forwarded-Email";
const X_FORWARDED_ISSUER: &str = "X-Forwarded-Issuer";
const DANGER_JWT_OVERRIDE: &str = "DANGER_JWT_OVERRIDE";

#[derive(Serialize, Deserialize, Debug)]
pub struct Jwt {
    pub iss: String,
    pub sub: String,
    pub email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
}

impl<S> FromRequestParts<S> for Jwt
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let mut token_error: Option<String> = None;

        // 1) token override (env) or forwarded access token header
        if let Some(token) = access_token(parts) {
            match decode_jwt_payload(&token) {
                Ok(jwt) => return Ok(jwt),
                Err(err) => token_error = Some(err),
            }
        }

        // 2) fallback: forwarded user/email headers
        if let Some(jwt) = forwarded_identity(parts) {
            return Ok(jwt);
        }

        if let Some(err) = token_error {
            return Err((StatusCode::UNAUTHORIZED, err));
        }

        Err((
            StatusCode::UNAUTHORIZED,
            "Didn't find an authentication header".to_string(),
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
    let iss = header_str(parts, X_FORWARDED_ISSUER)?;
    let sub = header_str(parts, X_FORWARDED_USER)?;
    let email = header_str(parts, X_FORWARDED_EMAIL)?;
    Some(Jwt {
        iss: iss.to_owned(),
        sub: sub.to_owned(),
        email: email.to_owned(),
        given_name: None,
        family_name: None,
    })
}

fn decode_jwt_payload(token: &str) -> Result<Jwt, String> {
    let token = token.strip_prefix("Bearer ").unwrap_or(token);

    // JWT is "header.payload.signature"
    let payload_b64 = token
        .split('.')
        .nth(1)
        .ok_or_else(|| "JWT payload segment is missing".to_string())?;
    let payload = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|_| "JWT payload is not valid base64url".to_string())?;
    let value: Value = serde_json::from_slice(&payload)
        .map_err(|_| "JWT payload is not valid JSON".to_string())?;

    let iss = required_claim(&value, "iss")?;
    let sub = required_claim(&value, "sub")?;
    let email = required_claim(&value, "email")?;
    let given_name = optional_claim(&value, "given_name");
    let family_name = optional_claim(&value, "family_name");

    Ok(Jwt {
        iss,
        sub,
        email,
        given_name,
        family_name,
    })
}

fn required_claim(value: &Value, field: &str) -> Result<String, String> {
    let Some(claim) = value.get(field) else {
        return Err(format!("JWT missing required claim: {field}"));
    };
    let Some(claim) = claim.as_str() else {
        return Err(format!("JWT claim '{field}' must be a string"));
    };
    if claim.is_empty() {
        return Err(format!("JWT claim '{field}' cannot be empty"));
    }
    Ok(claim.to_owned())
}

fn optional_claim(value: &Value, field: &str) -> Option<String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}
