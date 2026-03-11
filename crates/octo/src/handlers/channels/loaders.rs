use crate::{CustomError, Jwt, authz, handlers};
use axum::{Extension, response::Html};
use clorinde::deadpool_postgres::Pool;
use octo_ui::channels::page;
use octo_ui::routes;

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
    let agents = clorinde::queries::agents::list_my_agents()
        .bind(&transaction, &org_id)
        .all()
        .await?;
    let balance_label = handlers::load_balance_label(&transaction, &org_id).await?;

    transaction.commit().await?;

    let html = page::page(
        org_id,
        balance_label,
        channels,
        channel_setup.configured,
        agents,
        None,
        None,
    );
    Ok(Html(html))
}
