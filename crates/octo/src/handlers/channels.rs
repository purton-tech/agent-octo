use crate::{CustomError, Jwt, authz};
use axum::{Extension, response::Html};
use clorinde::deadpool_postgres::Pool;
use octo_ui::channels::pages;
use octo_ui::routes;

pub async fn loader(
    routes::channels::Index { org_id }: routes::channels::Index,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, &current_user).await?;
    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    let channels = clorinde::queries::channels_list::list_org_channels()
        .bind(&transaction, &org_id)
        .all()
        .await?;

    transaction.commit().await?;

    let html = pages::page(org_id, channels);
    Ok(Html(html))
}
