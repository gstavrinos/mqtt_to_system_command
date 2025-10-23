use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::{fs, process::Command, time::Duration};
use tokio::time;
mod structs;

fn run_command(command: &String) {
    let mut parts = command.split_whitespace();
    if let Some(program) = parts.next() {
        let args: Vec<&str> = parts.collect();
        let program = program.to_string();
        let args = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        tokio::spawn(async move {
            let status = Command::new(&program).args(&args).status();
            match status {
                Ok(s) => println!("Command exited with: {}", s),
                Err(e) => eprintln!("Failed to run command: {}", e),
            }
        });
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let yaml: String;
    if args.len() > 1 {
        yaml = fs::read_to_string(&args[1]).unwrap_or_default();
    } else {
        yaml = fs::read_to_string("config/conf.yaml").unwrap_or_default();
    }
    let config: structs::MQTT2SCConfig = serde_yaml::from_str(&yaml).unwrap_or_default();

    let mut mqttoptions = MqttOptions::new(
        config.mqtt_client_name.unwrap_or_default(),
        config.mqtt_host.unwrap_or_default(),
        config.mqtt_port.unwrap_or_default(),
    );
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    if !config.mqtt_password.clone().unwrap_or_default().is_empty() {
        mqttoptions.set_credentials(
            config.mqtt_username.unwrap_or_default(),
            config.mqtt_password.unwrap_or_default(),
        );
    }

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    let command_index_topic_clone = config.command_index_topic.unwrap_or_default().clone();
    let arbitrary_command_topic_clone = config.arbitrary_command_topic.unwrap_or_default().clone();
    let commands_clone = config.commands.unwrap_or_default().clone();
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                let payload = String::from_utf8_lossy(&p.payload).trim().to_string();
                let topic = p.topic.clone();
                if topic == command_index_topic_clone {
                    if let Ok(idx) = payload.parse::<usize>() {
                        if idx < commands_clone.len() {
                            let command = &commands_clone[idx];
                            println!("Running index {} -> {}", idx, command);
                            run_command(command);
                        } else {
                            eprintln!(
                                "Received index {} out of range (0..{})",
                                idx,
                                commands_clone.len().saturating_sub(1)
                            );
                        }
                    } else {
                        eprintln!("Invalid index payload: {}", payload);
                    }
                } else if topic == arbitrary_command_topic_clone {
                    run_command(&payload.clone());
                }
            }
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                eprintln!("Connected to the MQTT broker");
                if !(commands_clone.is_empty() || command_index_topic_clone.is_empty()) {
                    client
                        .subscribe(command_index_topic_clone.clone(), QoS::AtMostOnce)
                        .await?;
                    eprintln!("Subscribed to {}", command_index_topic_clone);
                }
                client
                    .subscribe(arbitrary_command_topic_clone.clone(), QoS::AtMostOnce)
                    .await?;
                eprintln!("Subscribed to {}", arbitrary_command_topic_clone);
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("MQTT error: {}", e);
                time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
