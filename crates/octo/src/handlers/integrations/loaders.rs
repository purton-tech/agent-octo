use crate::{CustomError, Jwt, authz, handlers};
use axum::{Extension, response::Html};
use clorinde::deadpool_postgres::Pool;
use octo_ui::integrations;
use octo_ui::routes;
use uuid::Uuid;

pub async fn loader(
    routes::integrations::Index { org_id }: routes::integrations::Index,
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

    let integrations = clorinde::queries::integrations::list_integrations()
        .bind(&transaction, &org_id)
        .all()
        .await?;
    let balance_label = handlers::load_balance_label(&transaction, &org_id).await?;

    transaction.commit().await?;

    let html = integrations::page::page(org_id, balance_label, integrations);
    Ok(Html(html))
}

pub async fn loader_new(
    routes::integrations::New { org_id }: routes::integrations::New,
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

    let html = integrations::upsert::page(org_id, balance_label, None, None, None);
    Ok(Html(html))
}

pub async fn loader_edit(
    routes::integrations::Edit { org_id, id }: routes::integrations::Edit,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Html<String>, CustomError> {
    let integration_id = Uuid::parse_str(&id)
        .map_err(|_| CustomError::FaultySetup("Invalid integration id".to_string()))?;

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, &current_user).await?;
    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    let integration = clorinde::queries::integrations::get_integration_for_edit()
        .bind(&transaction, &integration_id, &org_id)
        .opt()
        .await?
        .ok_or_else(|| CustomError::FaultySetup("Integration not found".to_string()))?;
    let balance_label = handlers::load_balance_label(&transaction, &org_id).await?;

    transaction.commit().await?;

    let html = integrations::upsert::page(org_id, balance_label, Some(integration), None, None);
    Ok(Html(html))
}
