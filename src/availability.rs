use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Availability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    payload_available: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    payload_not_available: Option<String>,
    topic: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value_template: Option<String>,
}

impl Availability {
    pub fn new(topic: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            ..Default::default()
        }
    }

    pub fn payload_available(mut self, payload: impl Into<String>) -> Self {
        self.payload_available = Some(payload.into());
        self
    }

    pub fn payload_not_available(mut self, payload: impl Into<String>) -> Self {
        self.payload_not_available = Some(payload.into());
        self
    }

    pub fn value_template(mut self, template: impl Into<String>) -> Self {
        self.value_template = Some(template.into());
        self
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(untagged, rename_all = "snake_case")]
pub enum AvailabilityMode {
    All,
    Any,
    Latest,
}

//Tests for the Availability struct
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_availability_new() {
        let availability = Availability::new("test/topic");
        assert_eq!(availability.topic, "test/topic");
        assert_eq!(availability.payload_available, None);
        assert_eq!(availability.payload_not_available, None);
        assert_eq!(availability.value_template, None);
    }

    #[test]
    fn test_availability_payload_available() {
        let availability = Availability::new("test/topic").payload_available("online");
        assert_eq!(availability.payload_available, Some("online".to_string()));
    }

    #[test]
    fn test_availability_payload_not_available() {
        let availability = Availability::new("test/topic").payload_not_available("offline");
        assert_eq!(
            availability.payload_not_available,
            Some("offline".to_string())
        );
    }

    #[test]
    fn test_availability_value_template() {
        let availability = Availability::new("test/topic").value_template("{{ value_json }}");
        assert_eq!(
            availability.value_template,
            Some("{{ value_json }}".to_string())
        );
    }
}
