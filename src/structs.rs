use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MQTT2SCConfig {
    pub mqtt_username: Option<String>,
    pub mqtt_password: Option<String>,
    pub mqtt_client_name: Option<String>,
    pub mqtt_host: Option<String>,
    pub mqtt_port: Option<u16>,
    pub arbitrary_command_topic: Option<String>,
    pub command_index_topic: Option<String>,
    pub commands: Option<Vec<String>>,
}

impl Default for MQTT2SCConfig {
    fn default() -> Self {
        Self {
            mqtt_username: Some(String::new()),
            mqtt_password: Some(String::new()),
            mqtt_client_name: Some(String::from("mqtt_to_system_command")),
            mqtt_host: Some(String::from("localhost")),
            mqtt_port: Some(1883),
            arbitrary_command_topic: Some(String::from("mqtt_to_system_command/command")),
            command_index_topic: Some(String::from("mqtt_to_system_command/command_index")),
            commands: None,
        }
    }
}
