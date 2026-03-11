use crate::{CustomError, Jwt, authz, handlers};
use axum::{
    Extension, Form,
    response::{Html, IntoResponse, Redirect, Response},
};
use clorinde::deadpool_postgres::Pool;
use octo_ui::channels::page::ConnectTelegramDraft;
use octo_ui::routes;
use serde::Deserialize;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct ConnectTelegramForm {
    #[validate(length(min = 1, message = "Bot token is required"))]
    pub bot_token: String,
    #[validate(custom(function = "validate_uuid"))]
    pub default_agent_id: String,
}

fn validate_uuid(value: &str) -> Result<(), ValidationError> {
    if Uuid::parse_str(value.trim()).is_ok() {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_uuid");
        err.message = Some("A valid default agent is required".into());
        Err(err)
    }
}

fn validation_message(form: &ConnectTelegramForm) -> Option<String> {
    if form.bot_token.trim().is_empty() {
        return Some("Bot token is required".to_string());
    }
    if Uuid::parse_str(form.default_agent_id.trim()).is_err() {
        return Some("A valid default agent is required".to_string());
    }
    None
}

async fn render_connect_error(
    pool: &Pool,
    current_user: &Jwt,
    org_id: String,
    form: &ConnectTelegramForm,
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

    let channel_setup = clorinde::queries::channels_list::has_telegram_channel()
        .bind(&transaction, &org_id)
        .one()
        .await?;

    let channels = clorinde::queries::channels_list::list_org_channels()
        .bind(&transaction, &org_id)
        .all()
        .await?;
    let agents = clorinde::queries::agents::list_my_agents()
        .bind(&transaction, &org_id)
        .all()
        .await?;
    let balance_label = handlers::load_balance_label(&transaction, &org_id).await?;

    transaction.commit().await?;

    let html = octo_ui::channels::page::page(
        org_id,
        balance_label,
        channels,
        channel_setup.configured,
        agents,
        Some(ConnectTelegramDraft {
            bot_token: form.bot_token.clone(),
            default_agent_id: form.default_agent_id.clone(),
        }),
        Some(message),
    );
    Ok(Html(html).into_response())
}

pub async fn action_connect_telegram(
    routes::channels::ConnectTelegram { org_id }: routes::channels::ConnectTelegram,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
    Form(form): Form<ConnectTelegramForm>,
) -> Result<Response, CustomError> {
    if let Err(errs) = form.validate() {
        let message = errs
            .field_errors()
            .values()
            .next()
            .and_then(|errs| errs.first())
            .and_then(|err| err.message.clone())
            .map(|m| m.to_string())
            .or_else(|| validation_message(&form))
            .unwrap_or_else(|| "Invalid form submission".to_string());
        return render_connect_error(&pool, &current_user, org_id, &form, message).await;
    }
    if let Some(message) = validation_message(&form) {
        return render_connect_error(&pool, &current_user, org_id, &form, message).await;
    }

    let bot_token = form.bot_token.trim().to_string();
    let default_agent_id = Uuid::parse_str(form.default_agent_id.trim())
        .map_err(|_| CustomError::FaultySetup("A valid default agent is required".to_string()))?;

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let context = authz::init_request(&transaction, &current_user).await?;

    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    clorinde::queries::channels_list::connect_telegram_channel()
        .bind(&transaction, &org_id, &bot_token, &default_agent_id)
        .one()
        .await?;

    transaction.commit().await?;

    let href = routes::agents::Index { org_id }.to_string();
    Ok(Redirect::to(&href).into_response())
}
