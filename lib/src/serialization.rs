pub mod mac_addr {
    use std::str::FromStr;

    use macaddr::MacAddr6;
    use serde::{Deserializer, Serializer, de::Visitor};

    pub fn serialize<S: Serializer>(
        mac_address: &MacAddr6,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&mac_address.to_string())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<MacAddr6, D::Error> {
        deserializer.deserialize_str(MacAddr6Visitor)
    }

    struct MacAddr6Visitor;

    impl<'de> Visitor<'de> for MacAddr6Visitor {
        type Value = MacAddr6;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a 6 byte mac address formatted as 00:00:00:00:00:00")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            MacAddr6::from_str(v).map_err(|err| E::custom(err))
        }

        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            MacAddr6::from_str(v).map_err(|err| E::custom(err))
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            MacAddr6::from_str(&v).map_err(|err| E::custom(err))
        }
    }
}
