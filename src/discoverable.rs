use serde::Serialize;

#[allow(clippy::declare_interior_mutable_const)]
const PREFIX: std::cell::OnceCell<&str> = std::cell::OnceCell::new();

pub trait ObjectId {
    fn object_id(&self) -> &str;
}

pub trait Component {
    fn component(&self) -> crate::component::Component;
}

pub trait NodeId {
    fn node_id(&self) -> Option<&str>;
}

pub trait Discoverable: serde::Serialize {
    fn prefix(&self) -> &'static str {
        #[allow(clippy::borrow_interior_mutable_const)]
        PREFIX.get_or_init(|| option_env!("HA_MQTT_PREFIX").unwrap_or("homeassistant"))
    }

    fn config_topic(&self) -> String;
}

pub trait State {
    fn state_topic(&self) -> String;
}

impl<T> Discoverable for T
where
    T: Component + ObjectId + NodeId + Serialize,
{
    fn config_topic(&self) -> String {
        if let Some(node_id) = self.node_id() {
            format!(
                "{}/{}/{}/{}/config",
                self.prefix().to_lowercase(),
                self.component(),
                node_id.to_lowercase(),
                self.object_id().to_lowercase()
            )
        } else {
            format!(
                "{}/{}/{}/config",
                self.prefix().to_lowercase(),
                self.component(),
                self.object_id().to_lowercase()
            )
        }
    }
}

impl<T> State for T
where
    T: Component + ObjectId + NodeId + Serialize,
{
    fn state_topic(&self) -> String {
        if let Some(node_id) = self.node_id() {
            format!(
                "{}/{}/{}/{}/state",
                self.prefix().to_lowercase(),
                self.component(),
                node_id.to_lowercase(),
                self.object_id().to_lowercase()
            )
        } else {
            format!(
                "{}/{}/{}/state",
                self.prefix().to_lowercase(),
                self.component(),
                self.object_id().to_lowercase()
            )
        }
    }
}
