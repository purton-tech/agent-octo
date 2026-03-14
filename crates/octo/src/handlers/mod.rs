pub mod agents;
pub mod billing;
pub mod channels;
pub mod connections;
pub mod integrations;
pub mod providers;
pub mod root;

use crate::CustomError;
use clorinde::queries::orgs;
use clorinde::tokio_postgres::Transaction;

pub async fn load_balance_label(
    transaction: &Transaction<'_>,
    org_id: &str,
) -> Result<String, CustomError> {
    let balance = orgs::get_org_balance()
        .bind(transaction, &org_id)
        .one()
        .await?;
    Ok(format_balance_microcents(balance.balance_microcents))
}

pub fn format_balance_microcents(balance_microcents: i64) -> String {
    let is_negative = balance_microcents < 0;
    let abs_microcents = balance_microcents.unsigned_abs();
    let cents = abs_microcents / 1_000_000;
    let dollars = cents / 100;
    let cents_remainder = cents % 100;

    if is_negative {
        format!("-${dollars}.{cents_remainder:02}")
    } else {
        format!("${dollars}.{cents_remainder:02}")
    }
}
