use crate::{CustomError, Jwt, authz, config::Config, handlers, stripe};
use axum::{Extension, extract::Query, response::Html};
use clorinde::deadpool_postgres::Pool;
use clorinde::queries::billing::TopUpTransaction;
use clorinde::tokio_postgres::Transaction;
use octo_ui::billing::page;
use octo_ui::routes;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Default)]
pub struct BillingQuery {
    pub top_up_transaction_id: Option<String>,
}

pub async fn loader(
    routes::billing::Index { org_id }: routes::billing::Index,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
    Query(query): Query<BillingQuery>,
    current_user: Jwt,
) -> Result<Html<String>, CustomError> {
    let mut error_message = None;

    if let Some(transaction_id) = query.top_up_transaction_id.as_deref()
        && let Err(err) =
            reconcile_returned_top_up(&pool, &config, &org_id, transaction_id, &current_user).await
    {
        error_message = Some(err.to_string());
    }

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (balance_label, history) =
        load_billing_page_data(&transaction, &org_id, &current_user).await?;
    transaction.commit().await?;

    Ok(Html(page::page(
        org_id,
        balance_label,
        history,
        error_message,
    )))
}

pub async fn load_billing_page_data(
    transaction: &Transaction<'_>,
    org_id: &str,
    current_user: &Jwt,
) -> Result<(String, Vec<TopUpTransaction>), CustomError> {
    let context = authz::init_request(transaction, current_user).await?;
    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    let balance_label = handlers::load_balance_label(transaction, org_id).await?;
    let history = clorinde::queries::billing::list_top_up_transactions()
        .bind(transaction, &org_id)
        .all()
        .await?;

    Ok((balance_label, history))
}

async fn reconcile_returned_top_up(
    pool: &Pool,
    config: &Config,
    org_id: &str,
    transaction_id: &str,
    current_user: &Jwt,
) -> Result<(), CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, current_user).await?;
    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    let transaction_id = Uuid::parse_str(transaction_id).map_err(|_| {
        CustomError::FaultySetup("Invalid top-up transaction id in return URL".to_string())
    })?;

    let top_up = clorinde::queries::billing::get_top_up_transaction_for_org()
        .bind(&transaction, &transaction_id, &org_id)
        .opt()
        .await?;
    transaction.commit().await?;

    let Some(top_up) = top_up else {
        return Ok(());
    };

    if top_up.status != "pending" || top_up.stripe_checkout_session_id.is_empty() {
        return Ok(());
    }

    let checkout_session =
        stripe::fetch_checkout_session(config, &top_up.stripe_checkout_session_id).await?;

    if checkout_session.payment_status.as_deref() != Some("paid") {
        return Ok(());
    }

    let payment_intent = checkout_session
        .payment_intent
        .as_ref()
        .map(|value| value.id().to_string());

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    clorinde::queries::billing::complete_top_up_checkout_session()
        .bind(
            &transaction,
            &payment_intent,
            &format!("return-reconcile:{}", checkout_session.id),
            &checkout_session.id,
        )
        .one()
        .await?;
    transaction.commit().await?;

    Ok(())
}
