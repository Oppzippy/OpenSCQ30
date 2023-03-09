use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use tracing::Level;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Display, EnumString, EnumIter)]
pub enum LoggingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LoggingLevel {
    pub fn allowed_values_string() -> String {
        let allowed_values = Self::iter()
            .map(|level| heck::AsKebabCase(level.to_string()).to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{allowed_values}]")
    }
}

impl From<LoggingLevel> for Level {
    fn from(level: LoggingLevel) -> Self {
        match level {
            LoggingLevel::Trace => Self::TRACE,
            LoggingLevel::Debug => Self::DEBUG,
            LoggingLevel::Info => Self::INFO,
            LoggingLevel::Warn => Self::WARN,
            LoggingLevel::Error => Self::ERROR,
        }
    }
}
