# MQTT to system command

Trigger system command execution through MQTT.

## Two modes of operation

- Arbitrary command mode: The message that is sent is executed as a system command
- Command index mode: The message sent contains an unsigned integer that is used as an index for a predefined commands arrays.

## Configuration parameters

- `mqtt_username`: String that contains the username in case the MQTT connection requires credentials. Empty by default
- `mqtt_password`: String that contains the password in case the MQTT connection requires credentials. Empty by default

* `mqtt_client_name`: String that contains the name of the client to be connected to the MQTT broker. Defaults to `mqtt_to_system_command`
* `mqtt_host`: String the contains the broker's IP or hostname. Defaults to `localhost`
* `mqtt_port`: Integer that contains the MQTT broker's port. Defaults to `1883`
* `arbitrary_command_topic`: String that contains the topic to listen to for arbitrary commands (The first mode). Defaults to `mqtt_to_system_command/command`
* `command_index_topic`: String that contains the topic to listen to for a command index. Defaults to `mqtt_to_system_command/command_index`
* `commands`: Vector of strings that contain the predefined commands to be executed based on the received index. Empty by default. The `command_index_topic` is not subscribed to if the `commands` parameter is empty

Note: Currently, the input configuration `yaml` file is not configurable. The code expects to find a configuration file in `config/conf.yaml`.
