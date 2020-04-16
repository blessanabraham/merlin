use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    host: String,
    port: u16,
    service: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::new();

        let default = include_str!("./settings.json");
        settings.merge(File::from_str(default, FileFormat::Json))?;

        settings.merge(Environment::default().separator("_").ignore_empty(false))?;

        settings.try_into()
    }
}
