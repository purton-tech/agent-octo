use crate::{CustomError, Jwt, authz};
use axum::{
    Extension, Form,
    response::{Html, Redirect},
};
use clorinde::deadpool_postgres::Pool;
use octo_ui::providers::pages;
use octo_ui::routes;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateProviderForm {
    pub provider_kind: String,
    pub api_key: String,
}

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

    transaction.commit().await?;

    let html = pages::index_page(org_id, providers);
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

    transaction.commit().await?;
    let html = pages::new_page(org_id);
    Ok(Html(html))
}

pub async fn action_create(
    routes::providers::Create { org_id }: routes::providers::Create,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
    Form(form): Form<CreateProviderForm>,
) -> Result<Redirect, CustomError> {
    let provider_kind = form.provider_kind.trim().to_string();
    let api_key = form.api_key.trim().to_string();

    if provider_kind.is_empty() || api_key.is_empty() {
        return Err(CustomError::FaultySetup(
            "Provider kind and API key are required".to_string(),
        ));
    }

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let context = authz::init_request(&transaction, &current_user).await?;

    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    let setup = clorinde::queries::providers::create_provider_connection()
        .bind(&transaction, &provider_kind, &org_id, &api_key)
        .one()
        .await?;

    if !setup.configured {
        return Err(CustomError::FaultySetup(format!(
            "Provider '{provider_kind}' was not connected. Make sure it is a supported provider and there is at least one agent without provider configuration."
        )));
    }

    transaction.commit().await?;

    let href = routes::providers::Index { org_id }.to_string();
    Ok(Redirect::to(&href))
}
