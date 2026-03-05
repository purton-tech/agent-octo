use crate::{CustomError, Jwt, authz};
use axum::{
    Extension,
    response::{Html, Redirect},
};
use clorinde::deadpool_postgres::Pool;
use octo_ui::root;
use octo_ui::routes;

pub async fn home(
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Redirect, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, &current_user).await?;
    transaction.commit().await?;

    let href = routes::users::Index {
        org_id: context.org_id,
    }
    .to_string();
    Ok(Redirect::to(&href))
}

pub async fn loader(
    routes::users::Index { org_id }: routes::users::Index,
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

    let current_user = clorinde::queries::auth::get_current_user()
        .bind(&transaction)
        .one()
        .await?;

    if context.user_id != current_user.id.to_string() {
        return Err(CustomError::FaultySetup(
            "Auth claim mismatch for current user".to_string(),
        ));
    }

    transaction.commit().await?;

    let users = vec![clorinde::queries::auth::User {
        id: current_user.id,
        email: context.email,
    }];
    let html = root::index(org_id, users);

    Ok(Html(html))
}
