use crate::{CustomError, Jwt, authz, handlers};
use axum::{Extension, response::Html};
use clorinde::deadpool_postgres::Pool;
use clorinde::queries::billing::TopUpTransaction;
use clorinde::tokio_postgres::Transaction;
use octo_ui::billing::page;
use octo_ui::routes;

pub async fn loader(
    routes::billing::Index { org_id }: routes::billing::Index,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (balance_label, history) =
        load_billing_page_data(&transaction, &org_id, &current_user).await?;
    transaction.commit().await?;

    Ok(Html(page::page(org_id, balance_label, history, None)))
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
