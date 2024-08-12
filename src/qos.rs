use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Qos {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

impl Serialize for Qos {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Qos {
    fn deserialize<D>(deserializer: D) -> Result<Qos, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let qos = u8::deserialize(deserializer)?;
        match qos {
            0 => Ok(Qos::AtMostOnce),
            1 => Ok(Qos::AtLeastOnce),
            2 => Ok(Qos::ExactlyOnce),
            _ => Err(serde::de::Error::custom("Invalid QoS value")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qos_serialization() {
        let qos = Qos::AtMostOnce;
        let serialized = serde_json::to_string(&qos).unwrap();
        assert_eq!(serialized, "0");
    }

    #[test]
    fn test_qos_deserialization() {
        let serialized = r#"2"#;
        let deserialized: Qos = serde_json::from_str(serialized).unwrap();
        assert_eq!(deserialized, Qos::ExactlyOnce);
    }
}
