#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub app_base_url: String,
    pub stripe_secret_key: Option<String>,
    pub stripe_webhook_secret: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        let database_url = std::env::var("APPLICATION_URL").expect("APPLICATION_URL not set");
        let app_base_url =
            std::env::var("APP_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").ok();
        let stripe_webhook_secret = std::env::var("STRIPE_WEBHOOK_SECRET").ok();

        Config {
            database_url,
            app_base_url,
            stripe_secret_key,
            stripe_webhook_secret,
        }
    }
}
