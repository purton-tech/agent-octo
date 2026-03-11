use crate::{CustomError, Jwt, authz, handlers};
use axum::{
    Extension, Form,
    response::{Html, IntoResponse, Redirect, Response},
};
use clorinde::deadpool_postgres::Pool;
use clorinde::types::ResourceVisibility;
use octo_ui::connections::r#new::CreateConnectionDraft;
use octo_ui::routes;
use serde::Deserialize;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct CreateConnectionForm {
    #[validate(custom(function = "validate_uuid"))]
    pub integration_id: String,
    #[validate(custom(function = "validate_visibility"))]
    pub visibility: String,
    pub api_key: String,
}

fn validate_uuid(value: &str) -> Result<(), ValidationError> {
    if Uuid::parse_str(value.trim()).is_ok() {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_uuid");
        err.message = Some("Select a valid integration".into());
        Err(err)
    }
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

fn parse_visibility(value: &str) -> Result<ResourceVisibility, String> {
    match value {
        "private" => Ok(ResourceVisibility::private),
        "org" => Ok(ResourceVisibility::org),
        _ => Err("Visibility must be either 'private' or 'org'".to_string()),
    }
}

async fn render_new_error(
    pool: &Pool,
    current_user: &Jwt,
    org_id: String,
    form: &CreateConnectionForm,
    message: String,
) -> Result<Response, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, current_user).await?;
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

    let html = octo_ui::connections::r#new::page(
        org_id,
        balance_label,
        integrations,
        Some(CreateConnectionDraft {
            integration_id: form.integration_id.clone(),
            visibility: form.visibility.clone(),
            api_key: form.api_key.clone(),
        }),
        Some(message),
    );
    Ok(Html(html).into_response())
}

pub async fn action_create(
    routes::connections::Create { org_id }: routes::connections::Create,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
    Form(form): Form<CreateConnectionForm>,
) -> Result<Response, CustomError> {
    if let Err(errs) = form.validate() {
        let message = errs
            .field_errors()
            .values()
            .next()
            .and_then(|errs| errs.first())
            .and_then(|err| err.message.clone())
            .map(|m| m.to_string())
            .unwrap_or_else(|| "Invalid form submission".to_string());
        return render_new_error(&pool, &current_user, org_id, &form, message).await;
    }

    let integration_id = Uuid::parse_str(form.integration_id.trim())
        .map_err(|_| CustomError::FaultySetup("Select a valid integration".to_string()))?;
    let visibility = match parse_visibility(form.visibility.trim()) {
        Ok(v) => v,
        Err(message) => {
            return render_new_error(&pool, &current_user, org_id, &form, message).await;
        }
    };

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, &current_user).await?;
    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    let requirement =
        clorinde::queries::integration_connections::get_integration_auth_requirement()
            .bind(&transaction, &integration_id, &org_id)
            .opt()
            .await?
            .ok_or_else(|| CustomError::FaultySetup("Integration not found".to_string()))?;

    let api_key_secret_ref = if requirement.requires_auth {
        let api_key = form.api_key.trim().to_string();
        if api_key.is_empty() {
            return render_new_error(
                &pool,
                &current_user,
                org_id,
                &form,
                "API key is required for this integration".to_string(),
            )
            .await;
        }
        api_key
    } else {
        "not-required".to_string()
    };

    let created = clorinde::queries::integration_connections::create_integration_connection()
        .bind(
            &transaction,
            &org_id,
            &integration_id,
            &visibility,
            &api_key_secret_ref,
        )
        .one()
        .await?;

    if !created.changed {
        return Err(CustomError::FaultySetup(
            "Connection was not created".to_string(),
        ));
    }

    transaction.commit().await?;

    let href = routes::connections::Index { org_id }.to_string();
    Ok(Redirect::to(&href).into_response())
}
