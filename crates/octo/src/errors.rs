use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use clorinde::deadpool_postgres::PoolError;
use clorinde::tokio_postgres::Error as TokioPostgresError;
use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum CustomError {
    FaultySetup(String),
    Database(String),
}

// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::FaultySetup(ref cause) => write!(f, "Setup Error: {}", cause),
            //CustomError::Unauthorized(ref cause) => write!(f, "Setup Error: {}", cause),
            CustomError::Database(ref cause) => {
                write!(f, "Database Error: {}", cause)
            }
        }
    }
}

// So that errors get printed to the browser?
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::FaultySetup(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        };

        tracing::error!(
            "request failed: status={}, message={}",
            status,
            error_message
        );
        format!("status = {}, message = {}", status, error_message).into_response()
    }
}

impl From<axum::http::uri::InvalidUri> for CustomError {
    fn from(err: axum::http::uri::InvalidUri) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<TokioPostgresError> for CustomError {
    fn from(err: TokioPostgresError) -> CustomError {
        let message = if let Some(db_err) = err.as_db_error() {
            let mut parts = vec![
                format!(
                    "postgres error {}: {}",
                    db_err.code().code(),
                    db_err.message()
                ),
                format!("severity={}", db_err.severity()),
            ];
            if let Some(detail) = db_err.detail() {
                parts.push(format!("detail={detail}"));
            }
            if let Some(hint) = db_err.hint() {
                parts.push(format!("hint={hint}"));
            }
            if let Some(where_) = db_err.where_() {
                parts.push(format!("where={where_}"));
            }
            parts.join(", ")
        } else {
            let mut parts = vec![format!("postgres driver error: {err}")];
            let mut source = err.source();
            while let Some(src) = source {
                parts.push(format!("caused by: {src}"));
                source = src.source();
            }
            parts.join(", ")
        };

        CustomError::Database(message)
    }
}

impl From<PoolError> for CustomError {
    fn from(err: PoolError) -> CustomError {
        CustomError::Database(err.to_string())
    }
}
