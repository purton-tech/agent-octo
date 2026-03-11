use crate::{CustomError, Jwt, authz, handlers};
use axum::{
    Extension, Form,
    response::{Html, IntoResponse, Redirect, Response},
};
use clorinde::deadpool_postgres::Pool;
use clorinde::types::ResourceVisibility;
use oas3::OpenApiV3Spec;
use octo_ui::integrations::upsert::UpsertDraft;
use octo_ui::routes;
use serde::Deserialize;
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct UpsertIntegrationForm {
    pub id: Option<String>,
    #[validate(custom(function = "validate_visibility"))]
    pub visibility: String,
    #[validate(length(min = 1, message = "OpenAPI spec is required"))]
    pub openapi_spec: String,
}

fn validate_visibility(value: &str) -> Result<(), ValidationError> {
    match value {
        "private" | "org" => Ok(()),
        _ => {
            let mut err = ValidationError::new("invalid_visibility");
            err.message = Some("Visibility must be either 'private' or 'org'".into());
            Err(err)
        }
    }
}

fn validation_message(errors: &ValidationErrors) -> String {
    for (field, field_errors) in errors.field_errors() {
        if let Some(err) = field_errors.first() {
            if let Some(message) = &err.message {
                return message.to_string();
            }
            return format!("Invalid value for '{field}'");
        }
    }
    "Invalid form submission".to_string()
}

fn parse_visibility(value: &str) -> Result<ResourceVisibility, String> {
    match value {
        "private" => Ok(ResourceVisibility::private),
        "org" => Ok(ResourceVisibility::org),
        _ => Err("Visibility must be either 'private' or 'org'".to_string()),
    }
}

fn normalize_openapi_spec(raw: &str) -> Result<String, String> {
    let spec: OpenApiV3Spec =
        oas3::from_yaml(raw).map_err(|err| format!("Invalid OpenAPI specification: {err}"))?;
    if spec.info.title.trim().is_empty() {
        return Err("OpenAPI info.title is required".to_string());
    }

    serde_json::to_string_pretty(&spec).map_err(|err| format!("Failed to serialize spec: {err}"))
}

fn render_upsert_error(
    org_id: String,
    balance_label: String,
    form: &UpsertIntegrationForm,
    message: String,
) -> Response {
    let html = octo_ui::integrations::upsert::page(
        org_id,
        balance_label,
        None,
        Some(UpsertDraft {
            id: form.id.clone(),
            visibility: form.visibility.clone(),
            openapi_spec: form.openapi_spec.clone(),
        }),
        Some(message),
    );
    Html(html).into_response()
}

pub async fn action_upsert(
    routes::integrations::Upsert { org_id }: routes::integrations::Upsert,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
    Form(form): Form<UpsertIntegrationForm>,
) -> Result<Response, CustomError> {
    if let Err(errors) = form.validate() {
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
        return Ok(render_upsert_error(
            org_id,
            balance_label,
            &form,
            validation_message(&errors),
        ));
    }

    let visibility = match parse_visibility(form.visibility.trim()) {
        Ok(v) => v,
        Err(message) => {
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
            return Ok(render_upsert_error(org_id, balance_label, &form, message));
        }
    };
    let normalized_spec = match normalize_openapi_spec(form.openapi_spec.trim()) {
        Ok(spec) => spec,
        Err(message) => {
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
            return Ok(render_upsert_error(org_id, balance_label, &form, message));
        }
    };
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
    Ok(Redirect::to(&href).into_response())
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
