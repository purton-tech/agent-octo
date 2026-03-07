use crate::{CustomError, Jwt};
use clorinde::queries::{agents, auth};
use clorinde::tokio_postgres::Transaction;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub org_id: String,
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
        .bind(transaction, &jwt.sub)
        .one()
        .await?;
    auth::set_request_claim_iss()
        .bind(transaction, &jwt.iss)
        .one()
        .await?;

    agents::ensure_default_agent_for_user()
        .bind(transaction, &org.org_id)
        .one()
        .await?;

    Ok(RequestContext {
        org_id: org.org_public_id,
    })
}
