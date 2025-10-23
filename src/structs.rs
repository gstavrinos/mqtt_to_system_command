use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MQTT2SCConfig {
    #[serde(default)]
    pub mqtt_username: Option<String>,
    #[serde(default)]
    pub mqtt_password: Option<String>,
    #[serde(default)]
    pub mqtt_client_name: Option<String>,
    #[serde(default)]
    pub mqtt_host: Option<String>,
    #[serde(default)]
    pub mqtt_port: Option<u16>,
    #[serde(default)]
    pub arbitrary_command_topic: Option<String>,
    #[serde(default)]
    pub command_index_topic: Option<String>,
    #[serde(default)]
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
