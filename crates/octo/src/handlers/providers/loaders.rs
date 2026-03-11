use crate::{CustomError, Jwt, authz, handlers};
use axum::{Extension, response::Html};
use clorinde::deadpool_postgres::Pool;
use octo_ui::providers::{r#new, page};
use octo_ui::routes;

pub async fn loader(
    routes::providers::Index { org_id }: routes::providers::Index,
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

    let providers = clorinde::queries::providers::list_provider_connections()
        .bind(&transaction, &org_id)
        .all()
        .await?;
    let balance_label = handlers::load_balance_label(&transaction, &org_id).await?;

    transaction.commit().await?;

    let html = page::page(org_id, balance_label, providers);
    Ok(Html(html))
}

pub async fn loader_new(
    routes::providers::New { org_id }: routes::providers::New,
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

    let balance_label = handlers::load_balance_label(&transaction, &org_id).await?;
    transaction.commit().await?;
    let html = r#new::page(org_id, balance_label, None, None);
    Ok(Html(html))
}
