use crate::{CustomError, Jwt};
use clorinde::queries::auth;
use clorinde::tokio_postgres::Transaction;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub user_id: String,
    pub org_id: String,
    pub email: String,
}

pub async fn init_request(
    transaction: &Transaction<'_>,
    jwt: &Jwt,
) -> Result<RequestContext, CustomError> {
    let user = auth::upsert_user_by_issuer_sub()
        .bind(
            transaction,
            &jwt.iss,
            &jwt.sub,
            &jwt.email,
            &jwt.given_name,
            &jwt.family_name,
        )
        .one()
        .await?;

    let user_id = user.id.to_string();
    let org_name = format!("{}'s Org", user.email);

    auth::ensure_default_org_membership_for_user()
        .bind(transaction, &user.id, &org_name)
        .one()
        .await?;

    let org = auth::get_first_org_for_user()
        .bind(transaction, &user.id)
        .one()
        .await?;

    auth::set_request_claim_sub()
        .bind(transaction, &user_id)
        .one()
        .await?;
    auth::set_request_claim_iss()
        .bind(transaction, &jwt.iss)
        .one()
        .await?;
    auth::set_request_claim_external_sub()
        .bind(transaction, &jwt.sub)
        .one()
        .await?;

    Ok(RequestContext {
        user_id,
        org_id: org.org_id.to_string(),
        email: user.email,
    })
}
