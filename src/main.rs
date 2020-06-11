use crate::client::MqttClient;
use std::thread;
use std::time::Duration;

mod client;

fn main() {
    let client = MqttClient::new().unwrap();
    for i in 0..1000 {
        client.publish(&i.to_string());
    }
    thread::sleep(Duration::from_secs(60));
}
