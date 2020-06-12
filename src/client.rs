use paho_mqtt as mqtt;
use futures::Future;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

pub struct MqttClient {
    sender: Sender<mqtt::Message>
}

impl MqttClient {
    pub fn new() -> Result<MqttClient, String> {
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri("tcp://localhost:1883")
            .persistence(mqtt::PersistenceType::None)
            .finalize();

        let cli = mqtt::AsyncClient::new(create_opts)
            .map_err(|e| format!("Error creating the client: {}", e))?;

        let conn_opts = mqtt::ConnectOptions::new();

        cli.connect(conn_opts).wait()
            .map_err(|e| format!("Unable to connect: {:?}", e))?;

        let (tx, rx) = channel::<mqtt::Message>();

        thread::spawn(move || {
            loop {
                let msg = rx.recv().unwrap();
                thread::sleep(Duration::from_secs(1));
                let tok = cli.publish(msg.clone());
                match tok.wait() {
                    Ok(_) => println!("Message {} sent to mqtt with success", msg),
                    Err(e) => eprintln!("Error sending message: {:?}", e)
                }
            }
        });

        Ok(MqttClient {
            sender: tx
        })
    }

    pub fn publish(&self, msg: &str) {
        let mqtt_msg = mqtt::Message::new("test", msg, 0);
        match self.sender.send(mqtt_msg) {
            Ok(_) => println!("Message <{}> sent to queue", msg),
            Err(e) => eprintln!("Error on send message <{}> : {}", msg, e),
        };
    }
}