use crate::{CustomError, Jwt, authz};
use axum::{
    Extension, Form,
    response::{Html, Redirect},
};
use clorinde::deadpool_postgres::Pool;
use clorinde::types::ResourceVisibility;
use oas3::OpenApiV3Spec;
use octo_ui::integrations;
use octo_ui::routes;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpsertIntegrationForm {
    pub id: Option<String>,
    pub visibility: String,
    pub openapi_spec: String,
}

fn parse_visibility(value: &str) -> Result<ResourceVisibility, CustomError> {
    match value {
        "private" => Ok(ResourceVisibility::private),
        "org" => Ok(ResourceVisibility::org),
        _ => Err(CustomError::FaultySetup(
            "Visibility must be either 'private' or 'org'".to_string(),
        )),
    }
}

fn normalize_openapi_spec(raw: &str) -> Result<String, CustomError> {
    let spec: OpenApiV3Spec = oas3::from_yaml(raw)
        .map_err(|err| CustomError::FaultySetup(format!("Invalid OpenAPI specification: {err}")))?;
    if spec.info.title.trim().is_empty() {
        return Err(CustomError::FaultySetup(
            "OpenAPI info.title is required".to_string(),
        ));
    }

    serde_json::to_string_pretty(&spec)
        .map_err(|err| CustomError::FaultySetup(format!("Failed to serialize spec: {err}")))
}

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

    transaction.commit().await?;

    let html = integrations::page::page(org_id, integrations);
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

    transaction.commit().await?;

    let html = integrations::upsert::page(org_id, None);
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

    transaction.commit().await?;

    let html = integrations::upsert::page(org_id, Some(integration));
    Ok(Html(html))
}

pub async fn action_upsert(
    routes::integrations::Upsert { org_id }: routes::integrations::Upsert,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
    Form(form): Form<UpsertIntegrationForm>,
) -> Result<Redirect, CustomError> {
    let visibility = parse_visibility(form.visibility.trim())?;
    let normalized_spec = normalize_openapi_spec(form.openapi_spec.trim())?;
    let normalized_spec_json: serde_json::Value =
        serde_json::from_str(&normalized_spec).map_err(|err| {
            CustomError::FaultySetup(format!("Failed to prepare OpenAPI spec JSON: {err}"))
        })?;

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, &current_user).await?;
    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    if let Some(id) = form
        .id
        .as_ref()
        .map(|value| value.trim())
        .filter(|v| !v.is_empty())
    {
        let integration_id = Uuid::parse_str(id)
            .map_err(|_| CustomError::FaultySetup("Invalid integration id".to_string()))?;

        let updated = clorinde::queries::integrations::update_integration()
            .bind(
                &transaction,
                &visibility,
                &normalized_spec_json,
                &integration_id,
                &org_id,
            )
            .one()
            .await?;

        if !updated.changed {
            return Err(CustomError::FaultySetup(
                "Integration was not updated".to_string(),
            ));
        }
    } else {
        let inserted = clorinde::queries::integrations::create_integration()
            .bind(&transaction, &org_id, &visibility, &normalized_spec_json)
            .one()
            .await?;

        if !inserted.changed {
            return Err(CustomError::FaultySetup(
                "Integration was not created".to_string(),
            ));
        }
    }

    transaction.commit().await?;

    let href = routes::integrations::Index { org_id }.to_string();
    Ok(Redirect::to(&href))
}

pub async fn action_delete(
    routes::integrations::Delete { org_id, id }: routes::integrations::Delete,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Redirect, CustomError> {
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

    clorinde::queries::integrations::delete_integration()
        .bind(&transaction, &integration_id, &org_id)
        .one()
        .await?;

    transaction.commit().await?;

    let href = routes::integrations::Index { org_id }.to_string();
    Ok(Redirect::to(&href))
}
