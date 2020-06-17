use paho_mqtt as mqtt;
use futures::Future;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use crate::queue::Queue;
use std::sync::{Mutex, Arc};
use std::time::Duration;

pub struct MqttClient {
    sender: Sender<String>,
}

impl MqttClient {
    pub fn new(queue: Box<dyn Queue + Send>) -> Result<MqttClient, String> {
        let mut cli = self::create_paho_client()?;
        let queue = Arc::new(Mutex::new(queue));
        let clone_queue = queue.clone();

        cli.set_connected_callback( move |cli_c| {
            let cli_clone = cli_c.clone();
            let clone_queue = clone_queue.clone();
            thread::spawn(move || {
                let mut queue = clone_queue.lock().unwrap();
                while !queue.is_empty() {
                    for msg in queue.consume_queue() {
                        let mqtt_msg = mqtt::Message::new("test", msg.clone(), 0);
                        let tok = cli_clone.publish(mqtt_msg);
                        match tok.wait() {
                            Ok(_) => println!("Message <{}> sent to mqtt with success", msg),
                            Err(e) => eprintln!("Error sending message: {:?}", e)
                        }
                    }
                }
            });
        });

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(10))
            .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(7200))
            .finalize();

        cli.connect(conn_opts).wait()
            .map_err(|e| format!("Unable to connect: {:?}", e))?;

        let (tx, rx) = channel::<String>();
        thread::spawn(move || {
            loop {
                let msg = rx.recv().unwrap();
                let mut queue = queue.lock().unwrap();
                if queue.is_empty() {
                    let mqtt_msg = mqtt::Message::new("test", msg.clone(), 0);
                    let tok = cli.publish(mqtt_msg);
                    match tok.wait() {
                        Ok(_) => println!("Message <{}> sent to mqtt with success", &msg),
                        Err(e) => {
                            eprintln!("Error sending message: {:?}", e);
                            match queue.add(msg.clone()) {
                                Ok(_) => println!("Message <{}> sent to queue", &msg),
                                Err(e) => eprintln!("Error on sent message to queue: {}", e),
                            }
                        }
                    }
                } else {
                    match queue.add(msg.clone()) {
                        Ok(_) => println!("Message <{}> sent to queue", &msg),
                        Err(e) => eprintln!("Error on sent message to queue: {}", e),
                    }
                }
            }
        });

        Ok(MqttClient {
            sender: tx,
        })
    }

    pub fn publish(&self, msg: &str) {
        match self.sender.send(msg.to_string()) {
            Ok(_) => println!("Message <{}> sent", msg),
            Err(e) => eprintln!("Error on send message <{}> : {}", msg, e),
        };
    }
}

fn create_paho_client() -> Result<mqtt::AsyncClient, String> {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("tcp://localhost:1883")
        .persistence(mqtt::PersistenceType::None)
        .finalize();

    let cli = mqtt::AsyncClient::new(create_opts)
        .map_err(|e| format!("Error creating the client: {}", e))?;

    Ok(cli)
}

