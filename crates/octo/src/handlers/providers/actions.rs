use crate::{CustomError, Jwt, authz, handlers};
use axum::{
    Extension, Form,
    response::{Html, IntoResponse, Redirect, Response},
};
use clorinde::deadpool_postgres::Pool;
use octo_ui::providers::r#new::CreateProviderDraft;
use octo_ui::routes;
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct CreateProviderForm {
    #[validate(custom(function = "validate_provider_kind"))]
    pub provider_kind: String,
    #[validate(length(min = 1, message = "API key is required"))]
    pub api_key: String,
}

fn validate_provider_kind(value: &str) -> Result<(), ValidationError> {
    match value {
        "openai" | "anthropic" | "gemini" => Ok(()),
        _ => {
            let mut err = ValidationError::new("invalid_provider_kind");
            err.message = Some("Choose a supported provider".into());
            Err(err)
        }
    }
}

fn render_new_provider_error(
    org_id: String,
    balance_label: String,
    form: &CreateProviderForm,
    message: String,
) -> Response {
    let html = octo_ui::providers::r#new::page(
        org_id,
        balance_label,
        Some(CreateProviderDraft {
            provider_kind: form.provider_kind.clone(),
            api_key: form.api_key.clone(),
        }),
        Some(message),
    );
    Html(html).into_response()
}

pub async fn action_create(
    routes::providers::Create { org_id }: routes::providers::Create,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
    Form(form): Form<CreateProviderForm>,
) -> Result<Response, CustomError> {
    if let Err(errs) = form.validate() {
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
        let message = errs
            .field_errors()
            .values()
            .next()
            .and_then(|errs| errs.first())
            .and_then(|err| err.message.clone())
            .map(|m| m.to_string())
            .unwrap_or_else(|| "Invalid form submission".to_string());
        return Ok(render_new_provider_error(
            org_id,
            balance_label,
            &form,
            message,
        ));
    }

    let provider_kind = form.provider_kind.trim().to_string();
    let api_key = form.api_key.trim().to_string();

    if provider_kind.is_empty() || api_key.is_empty() {
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
        return Ok(render_new_provider_error(
            org_id,
            balance_label,
            &form,
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
    Ok(Redirect::to(&href).into_response())
}
