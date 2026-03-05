use crate::{CustomError, Jwt, authz};
use axum::{Extension, response::Redirect};
use clorinde::deadpool_postgres::Pool;
use octo_ui::routes;

pub async fn home(
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Redirect, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, &current_user).await?;
    transaction.commit().await?;

    let href = routes::agents::Index {
        org_id: context.org_id,
    }
    .to_string();
    Ok(Redirect::to(&href))
}
