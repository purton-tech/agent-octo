use crate::{CustomError, Jwt, authz};
use axum::{Extension, response::Html};
use clorinde::deadpool_postgres::Pool;
use octo_ui::agents::pages;

pub async fn loader(
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _context = authz::init_request(&transaction, &current_user).await?;
    let agents = clorinde::queries::agents::list_my_agents()
        .bind(&transaction)
        .all()
        .await?;

    transaction.commit().await?;

    let html = pages::page(agents);
    Ok(Html(html))
}
