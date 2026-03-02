#[derive(Clone, Debug)]
pub struct Config {
    pub application_url: String,
    pub telegram_bot_token: String,
}

impl Config {
    pub fn new() -> Self {
        let application_url =
            std::env::var("APPLICATION_URL").expect("APPLICATION_URL not set");
        let telegram_bot_token =
            std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

        Self {
            application_url,
            telegram_bot_token,
        }
    }
}
