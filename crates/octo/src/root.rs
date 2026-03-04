use crate::{CustomError, Jwt};
use axum::{Extension, response::Html};
use clorinde::deadpool_postgres::Pool;

pub async fn loader(
    Extension(_pool): Extension<Pool>,
    _current_user: Jwt,
) -> Result<Html<String>, CustomError> {
    Ok(Html(
        "<html><body><h1>Hello, world!</h1></body></html>".to_string(),
    ))
}
