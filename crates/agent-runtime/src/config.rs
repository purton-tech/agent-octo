#[derive(Clone, Debug)]
pub struct Config {
    pub application_url: String,
    pub service_role_jwt: String,
    pub stack_api_url: String,
}

impl Config {
    pub fn new() -> Self {
        let application_url =
            std::env::var("APPLICATION_URL").expect("APPLICATION_URL not set");
        let service_role_jwt =
            std::env::var("SERVICE_ROLE_JWT").expect("SERVICE_ROLE_JWT not set");
        let stack_api_url = std::env::var("STACK_API_URL")
            .unwrap_or_else(|_| "http://host.docker.internal:30060".to_string());

        Self {
            application_url,
            service_role_jwt,
            stack_api_url,
        }
    }
}
