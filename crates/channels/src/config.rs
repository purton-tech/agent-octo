#[derive(Clone, Debug)]
pub struct Config {
    pub application_url: String,
    pub service_role_jwt: String,
    pub stack_api_url: String,
    pub telegram_bot_token: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let application_url = std::env::var("APPLICATION_URL").expect("APPLICATION_URL not set");
        let service_role_jwt = std::env::var("SERVICE_ROLE_JWT").expect("SERVICE_ROLE_JWT not set");
        let stack_api_url = std::env::var("STACK_API_URL")
            .unwrap_or_else(|_| "http://host.docker.internal:30060".to_string());
        let telegram_bot_token =
            std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

        Self {
            application_url,
            service_role_jwt,
            stack_api_url,
            telegram_bot_token,
        }
    }
}
