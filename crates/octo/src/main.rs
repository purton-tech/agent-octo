mod agents;
mod authz;
mod config;
mod errors;
mod jwt;
mod root;
mod static_files;

use std::net::SocketAddr;

use axum::{Extension, Router, routing::get};
use axum_extra::routing::RouterExt;
use clorinde::deadpool_postgres::Manager;
use clorinde::tokio_postgres::NoTls;
use tower_livereload::LiveReloadLayer;

pub use errors::CustomError;
pub use jwt::Jwt;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    let config = config::Config::new();

    let pg_config: clorinde::tokio_postgres::Config = config
        .database_url
        .parse()
        .expect("APPLICATION_URL is invalid");
    let manager = Manager::new(pg_config, NoTls);
    let pool = clorinde::deadpool_postgres::Pool::builder(manager)
        .build()
        .expect("Failed to build database pool");

    // build our application with a route
    let app = Router::new()
        .route("/", get(root::loader))
        .route("/agents", get(agents::loader))
        .typed_get(static_files::static_path)
        .layer(LiveReloadLayer::new())
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
