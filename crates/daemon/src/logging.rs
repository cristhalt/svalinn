use std::path::PathBuf;

use serde::Deserialize;
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// The default directory path where log files will be stored.
const DEFAULT_LOG_DIR: &str = "/var/log/svalinn/";

/// Represents the configuration for logging.
#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    /// The path to the directory where logs will be stored.
    #[serde(default = "default_log_dir_path")]
    dir: PathBuf,
    /// The level of logging.
    level: Option<String>,
}

/// Provides the default path for the log directory.
fn default_log_dir_path() -> PathBuf {
    PathBuf::from(DEFAULT_LOG_DIR)
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            dir: default_log_dir_path(),
            level: None,
        }
    }
}

/// Initializes the logging system based on the provided configuration.
///
/// This function sets up both file-based and console-based logging.
/// The logging level is determined by the `log_level` parameter, defaulting
/// to `LevelFilter::INFO` if not specified.
pub fn init_logging(conf: LoggingConfig) -> Result<(), LogDirError> {
    std::fs::create_dir_all(&conf.dir)?;

    let file_appender = RollingFileAppender::new(Rotation::DAILY, conf.dir, "daemon.log");
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_appender)
        .with_ansi(false);

    let console_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_target(true)
        .compact();

    const DEFAULT_LEVEL_FILTER: LevelFilter = LevelFilter::INFO;
    let env_filter_builder =
        EnvFilter::builder().with_default_directive(DEFAULT_LEVEL_FILTER.into());

    let env_filter = match conf.level {
        Some(level) => env_filter_builder.parse_lossy(level),
        None => env_filter_builder.from_env_lossy(),
    };

    tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(file_layer)
        .with(console_layer)
        .init();

    Ok(())
}

/// Represents an error that occurs when the log directory cannot be created.
#[derive(Debug, thiserror::Error)]
#[error("Failed to create the log directory")]
pub struct LogDirError(#[from] std::io::Error);
