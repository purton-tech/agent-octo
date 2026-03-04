use crate::{CustomError, Jwt};
use axum::{Extension, response::Html};
use clorinde::deadpool_postgres::Pool;
use octo_ui::root;

pub async fn loader(
    Extension(pool): Extension<Pool>,
    _current_user: Jwt,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;
    let users = clorinde::queries::auth::get_users()
        .bind(&client)
        .all()
        .await?;
    let html = root::index(users);

    Ok(Html(html))
}
