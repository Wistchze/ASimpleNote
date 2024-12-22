use config::Config;
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

impl ServerConfig {
    pub fn new() -> Self {
        // Load env
        dotenv().ok();

        let builder = Config::builder();
        
        builder
            .add_source(config::Environment::default())
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}
