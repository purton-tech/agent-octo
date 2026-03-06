use crate::{CustomError, Jwt, authz};
use axum::{
    Extension, Form,
    response::{Html, Redirect},
};
use clorinde::deadpool_postgres::Pool;
use octo_ui::channels::pages;
use octo_ui::routes;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConnectTelegramForm {
    pub bot_token: String,
}

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
    let channel_setup = clorinde::queries::channels_list::has_telegram_channel()
        .bind(&transaction, &org_id)
        .one()
        .await?;

    let channels = clorinde::queries::channels_list::list_org_channels()
        .bind(&transaction, &org_id)
        .all()
        .await?;

    transaction.commit().await?;

    let html = pages::page(org_id, channels, channel_setup.configured);
    Ok(Html(html))
}

pub async fn action_connect_telegram(
    routes::channels::ConnectTelegram { org_id }: routes::channels::ConnectTelegram,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
    Form(form): Form<ConnectTelegramForm>,
) -> Result<Redirect, CustomError> {
    let bot_token = form.bot_token.trim().to_string();
    if bot_token.is_empty() {
        return Err(CustomError::FaultySetup(
            "Bot token is required".to_string(),
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

    clorinde::queries::channels_list::connect_telegram_channel()
        .bind(&transaction, &org_id, &bot_token)
        .one()
        .await?;

    transaction.commit().await?;

    let href = routes::agents::Index { org_id }.to_string();
    Ok(Redirect::to(&href))
}
