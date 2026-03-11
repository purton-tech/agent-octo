use crate::{CustomError, Jwt, authz, handlers};
use axum::{Extension, response::Html};
use clorinde::deadpool_postgres::Pool;
use octo_ui::connections::{r#new, page};
use octo_ui::routes;

pub async fn loader(
    routes::connections::Index { org_id }: routes::connections::Index,
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

    let connections = clorinde::queries::integration_connections::list_integration_connections()
        .bind(&transaction, &org_id)
        .all()
        .await?;
    let balance_label = handlers::load_balance_label(&transaction, &org_id).await?;

    transaction.commit().await?;

    let html = page::page(org_id, balance_label, connections);
    Ok(Html(html))
}

pub async fn loader_new(
    routes::connections::New { org_id }: routes::connections::New,
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

    let integrations = clorinde::queries::integration_connections::list_connectable_integrations()
        .bind(&transaction, &org_id)
        .all()
        .await?;
    let balance_label = handlers::load_balance_label(&transaction, &org_id).await?;

    transaction.commit().await?;

    let html = r#new::page(org_id, balance_label, integrations, None, None);
    Ok(Html(html))
}
