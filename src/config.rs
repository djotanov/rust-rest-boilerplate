use std::env;
use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_max_size: u32,
    pub pool_min_idle: u32,
    pub pool_max_lifetime_seconds: u64,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

impl ApplicationConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut parsed_config = Config::new();
        parsed_config.merge(File::with_name("config/application.yml"))?;
        let profile = env::var("PROFILE").unwrap_or_else(|_| "dev".into());
        parsed_config.merge(File::with_name(&format!("config/application-{}.yml", profile)).required(false))?;
        parsed_config.merge(File::with_name("config/application-local.yml").required(false))?;
        parsed_config.merge(Environment::with_prefix("app"))?;
        parsed_config.try_into()
    }
}

impl ServerConfig {
    pub fn get_listen_address(&self) -> String {
        let mut listen_address = self.host.to_string();
        listen_address.push_str(":");
        listen_address.push_str(&self.port.to_string());
        listen_address.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_load_config() {
        // Given

        // When
        let conf = ApplicationConfig::new().unwrap();

        // Then
        assert_eq!(conf.server.host, "0.0.0.0");
        assert_eq!(conf.server.port, 8001);
    }
}
