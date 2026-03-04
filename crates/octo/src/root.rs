use crate::errors::CustomError;
use axum::{Extension, response::Html};
use clorinde::deadpool_postgres::Pool;

pub async fn loader(Extension(_pool): Extension<Pool>) -> Result<Html<String>, CustomError> {
    Ok(Html(
        "<html><body><h1>Hello, world!</h1></body></html>".to_string(),
    ))
}
