use std::sync::LazyLock;

use regex::Regex;
use uuid::Uuid;

impl From<btleplug::Error> for crate::Error {
    fn from(err: btleplug::Error) -> Self {
        match err {
            btleplug::Error::DeviceNotFound => crate::Error::DeviceNotFound {
                source: Box::new(err),
            },
            btleplug::Error::NotConnected => crate::Error::NotConnected {
                source: Box::new(err),
            },
            btleplug::Error::InvalidBDAddr(_) => crate::Error::DeviceNotFound {
                source: Box::new(err),
            },
            btleplug::Error::TimedOut(_) => crate::Error::NotConnected {
                source: Box::new(err),
            },
            btleplug::Error::Other(_) => {
                static NOT_CONNECTED: &str = "Not connected";
                static SERVICE_NOT_FOUND: LazyLock<Regex> =
                    LazyLock::new(|| Regex::new("Service with UUID (.*) not found.").unwrap());
                static CHARACTERISTIC_NOT_FOUND: LazyLock<Regex> = LazyLock::new(|| {
                    Regex::new("Characteristic with UUID (.*) not found.").unwrap()
                });
                // Other with string error messages is used a lot on linux
                let error_message = err.to_string();

                if let Some(captures) = SERVICE_NOT_FOUND.captures(&error_message) {
                    crate::Error::ServiceNotFound {
                        uuid: Uuid::try_from(
                            captures
                                .get(1)
                                .map(|capture| capture.as_str())
                                .unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                        source: Some(Box::new(err)),
                    }
                } else if let Some(captures) = CHARACTERISTIC_NOT_FOUND.captures(&error_message) {
                    crate::Error::CharacteristicNotFound {
                        uuid: Uuid::try_from(
                            captures
                                .get(1)
                                .map(|capture| capture.as_str())
                                .unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                        source: Some(Box::new(err)),
                    }
                } else if error_message == NOT_CONNECTED {
                    crate::Error::NotConnected {
                        source: Box::new(err),
                    }
                } else {
                    crate::Error::Other {
                        source: Box::new(err),
                    }
                }
            }
            _ => crate::Error::Other {
                source: Box::new(err),
            },
        }
    }
}
