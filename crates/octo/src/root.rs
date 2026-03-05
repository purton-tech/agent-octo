use crate::{CustomError, Jwt, authz};
use axum::{Extension, response::Html};
use clorinde::deadpool_postgres::Pool;
use octo_ui::root;

pub async fn loader(
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, &current_user).await?;

    let current_user = clorinde::queries::auth::get_current_user()
        .bind(&transaction)
        .one()
        .await?;

    if context.user_id != current_user.id.to_string() {
        return Err(CustomError::FaultySetup(
            "Auth claim mismatch for current user".to_string(),
        ));
    }

    transaction.commit().await?;

    let users = vec![clorinde::queries::auth::User {
        id: current_user.id,
        email: context.email,
    }];
    let html = root::index(users);

    Ok(Html(html))
}
