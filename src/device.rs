use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Device {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hw_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_area: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sw_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_device: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl Device {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_configuration_url(mut self, configuration_url: impl Into<String>) -> Self {
        self.configuration_url = Some(configuration_url.into());
        self
    }
    pub fn with_connections(mut self, connections: Vec<Connection>) -> Self {
        self.connections = Some(connections);
        self
    }
    pub fn with_hw_version(mut self, hw_version: impl Into<String>) -> Self {
        self.hw_version = Some(hw_version.into());
        self
    }
    pub fn with_identifiers(mut self, identifiers: Vec<String>) -> Self {
        self.identifiers = Some(identifiers);
        self
    }
    pub fn with_manufacturer(mut self, manufacturer: impl Into<String>) -> Self {
        self.manufacturer = Some(manufacturer.into());
        self
    }
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn with_serial_number(mut self, serial_number: impl Into<String>) -> Self {
        self.serial_number = Some(serial_number.into());
        self
    }
    pub fn with_suggested_area(mut self, suggested_area: impl Into<String>) -> Self {
        self.suggested_area = Some(suggested_area.into());
        self
    }
    pub fn with_sw_version(mut self, sw_version: impl Into<String>) -> Self {
        self.sw_version = Some(sw_version.into());
        self
    }
    pub fn with_via_device(mut self, via_device: impl Into<String>) -> Self {
        self.via_device = Some(via_device.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Connection {
    pub connection_type: String,
    pub identifier: String,
}

impl Connection {
    pub fn new(connection_type: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self {
            connection_type: connection_type.into(),
            identifier: identifier.into(),
        }
    }
}

impl Serialize for Connection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let tuple = format!("[{}, {}]", self.connection_type, self.identifier);
        serializer.serialize_str(&tuple)
    }
}

impl<'de> Deserialize<'de> for Connection {
    fn deserialize<D>(deserializer: D) -> Result<Connection, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tuple = <String>::deserialize(deserializer)?;
        let mut parts = tuple.trim_matches(|p| p == '[' || p == ']').split(", ");
        let connection_type = parts.next().unwrap();
        let identifier = parts.next().unwrap();
        Ok(Connection {
            connection_type: connection_type.to_string(),
            identifier: identifier.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_new() {
        let connection = Connection::new("type1", "id1");
        assert_eq!(connection.connection_type, "type1");
        assert_eq!(connection.identifier, "id1");
    }

    #[test]
    fn test_connection_serialize() {
        let connection = Connection::new("type1", "id1");
        let serialized = serde_json::to_string(&connection).unwrap();
        assert_eq!(serialized, r#""[type1, id1]""#);
    }

    #[test]
    fn test_connection_deserialize() {
        let json = r#""[type1, id1]""#;
        let connection: Connection = serde_json::from_str(json).unwrap();
        assert_eq!(connection.connection_type, "type1");
        assert_eq!(connection.identifier, "id1");
    }
}
