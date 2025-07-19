use serde::Deserialize;

use crate::{fw::FirewallRuleConfig, logging::LoggingConfig};

/// The path to the configuration file used by the daemon.
const CONF_FILE_PATH: &str = "/etc/svalinn/config.toml";

/// Represents the global configuration for the daemon.
#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    /// The logging configuration.
    #[serde(default)]
    pub log: LoggingConfig,
    #[serde(default)]
    pub rules: FirewallRuleConfig,
}

impl GlobalConfig {
    /// Creates a `GlobalConfig` instance by reading from the configuration file.
    pub fn from_file() -> Result<Self, ConfigFileError> {
        let file_content = std::fs::read_to_string(CONF_FILE_PATH)?;
        let conf = toml::from_str(&file_content)?;
        Ok(conf)
    }
}

/// Represents errors that can occur while reading the configuration file.
#[derive(Debug, thiserror::Error)]
pub enum ConfigFileError {
    #[error("Failed to open the configuration file \"{CONF_FILE_PATH}\"")]
    OpenFile(#[from] std::io::Error),
    #[error("Invalid configuration file")]
    InvalidConfig(#[from] toml::de::Error),
}
