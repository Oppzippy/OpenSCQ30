use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use tracing::Level;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Display, EnumString, EnumIter)]
pub enum LoggingLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl LoggingLevel {
    pub fn allowed_values_string() -> String {
        let allowed_values = Self::iter()
            .map(|level| level.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{allowed_values}]")
    }
}

impl From<LoggingLevel> for Level {
    fn from(level: LoggingLevel) -> Self {
        match level {
            LoggingLevel::TRACE => Self::TRACE,
            LoggingLevel::DEBUG => Self::DEBUG,
            LoggingLevel::INFO => Self::INFO,
            LoggingLevel::WARN => Self::WARN,
            LoggingLevel::ERROR => Self::ERROR,
        }
    }
}
