#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new() -> Config {
        let database_url = std::env::var("APPLICATION_URL").expect("APPLICATION_URL not set");

        Config { database_url }
    }
}
