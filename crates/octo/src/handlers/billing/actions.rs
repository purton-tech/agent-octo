use crate::{CustomError, Jwt, authz, config::Config, stripe};
use axum::{
    Extension, Form,
    body::Bytes,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use clorinde::deadpool_postgres::Pool;
use octo_ui::billing::page;
use octo_ui::routes;
use serde::Deserialize;

use super::loaders::load_billing_page_data;

const ALLOWED_TOP_UP_AMOUNTS_CENTS: [i64; 3] = [1000, 2500, 5000];
const MICRO_CENTS_PER_CENT: i64 = 1_000_000;

#[derive(Debug, Deserialize)]
pub struct StartCheckoutForm {
    pub amount_cents: i64,
}

fn render_billing_error(
    org_id: String,
    balance_label: String,
    history: Vec<clorinde::queries::billing::TopUpTransaction>,
    message: String,
) -> Response {
    Html(page::page(org_id, balance_label, history, Some(message))).into_response()
}

pub async fn action_start_checkout(
    routes::billing::StartCheckout { org_id }: routes::billing::StartCheckout,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
    current_user: Jwt,
    Form(form): Form<StartCheckoutForm>,
) -> Result<Response, CustomError> {
    if !ALLOWED_TOP_UP_AMOUNTS_CENTS.contains(&form.amount_cents) {
        let mut client = pool.get().await?;
        let transaction = client.transaction().await?;
        let (balance_label, history) =
            load_billing_page_data(&transaction, &org_id, &current_user).await?;
        transaction.commit().await?;
        return Ok(render_billing_error(
            org_id,
            balance_label,
            history,
            "Choose one of the available top-up amounts.".to_string(),
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

    let current_db_user = clorinde::queries::auth::get_current_user()
        .bind(&transaction)
        .one()
        .await?;
    let org_summary = clorinde::queries::orgs::get_org_billing_summary()
        .bind(&transaction, &org_id)
        .one()
        .await?;

    let stripe_customer = match clorinde::queries::billing::get_stripe_customer_for_org()
        .bind(&transaction, &org_id)
        .opt()
        .await?
    {
        Some(existing) => existing.stripe_customer_id,
        None => {
            let created =
                stripe::create_customer(&config, &current_user.email, &org_summary.name, &org_id)
                    .await?;
            clorinde::queries::billing::upsert_stripe_customer_for_org()
                .bind(&transaction, &org_id, &created.id)
                .one()
                .await?
                .stripe_customer_id
        }
    };

    let amount_microcents = form.amount_cents * MICRO_CENTS_PER_CENT;

    let created_transaction = clorinde::queries::billing::create_top_up_transaction()
        .bind(
            &transaction,
            &org_id,
            &current_db_user.id,
            &amount_microcents,
        )
        .one()
        .await?;
    let transaction_id = created_transaction.id;

    let billing_href = routes::billing::Index {
        org_id: org_id.clone(),
    }
    .to_string();
    let transaction_id_str = transaction_id.to_string();
    let success_url = format!(
        "{}{}?top_up_transaction_id={}",
        config.app_base_url.trim_end_matches('/'),
        billing_href,
        transaction_id
    );
    let cancel_url = format!(
        "{}{}",
        config.app_base_url.trim_end_matches('/'),
        billing_href
    );

    let checkout_session = match stripe::create_checkout_session(
        &config,
        stripe::CheckoutSessionRequest {
            customer_id: &stripe_customer,
            org_name: &org_summary.name,
            org_id: &org_id,
            transaction_id: &transaction_id_str,
            amount_cents: form.amount_cents,
            success_url: &success_url,
            cancel_url: &cancel_url,
        },
    )
    .await
    {
        Ok(session) => session,
        Err(err) => {
            clorinde::queries::billing::mark_top_up_transaction_failed()
                .bind(&transaction, &transaction_id)
                .await?;
            transaction.commit().await?;

            let mut client = pool.get().await?;
            let transaction = client.transaction().await?;
            let (balance_label, history) =
                load_billing_page_data(&transaction, &org_id, &current_user).await?;
            transaction.commit().await?;
            return Ok(render_billing_error(
                org_id,
                balance_label,
                history,
                err.to_string(),
            ));
        }
    };

    clorinde::queries::billing::attach_top_up_checkout_session()
        .bind(&transaction, &checkout_session.id, &transaction_id)
        .await?;

    transaction.commit().await?;

    Ok(Redirect::to(&checkout_session.url).into_response())
}

pub async fn action_stripe_webhook(
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, CustomError> {
    let signature = headers
        .get("Stripe-Signature")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| {
            CustomError::FaultySetup("Stripe-Signature header is missing".to_string())
        })?;

    let event = stripe::verify_webhook(&config, signature, &body)?;

    if event.event_type != "checkout.session.completed" {
        return Ok(StatusCode::OK);
    }

    if event.data.object.payment_status.as_deref() != Some("paid") {
        return Ok(StatusCode::OK);
    }

    let payment_intent = event
        .data
        .object
        .payment_intent
        .as_ref()
        .map(|value| value.id().to_string());

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    clorinde::queries::billing::complete_top_up_checkout_session()
        .bind(
            &transaction,
            &payment_intent,
            &event.id,
            &event.data.object.id,
        )
        .one()
        .await?;
    transaction.commit().await?;

    Ok(StatusCode::OK)
}
