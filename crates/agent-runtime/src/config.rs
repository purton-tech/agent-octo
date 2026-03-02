#[derive(Clone, Debug)]
pub struct Config {
    pub application_url: String,
}

impl Config {
    pub fn new() -> Self {
        let application_url =
            std::env::var("APPLICATION_URL").expect("APPLICATION_URL not set");

        Self { application_url }
    }
}
