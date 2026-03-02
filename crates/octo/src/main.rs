#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("octo=info"))
        .with_target(false)
        .init();

    channels::telegram::run().await
}
