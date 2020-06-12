use crate::client::MqttClient;
use std::thread;
use std::time::Duration;

mod client;

fn main() {
    let client = MqttClient::new().unwrap();
    for i in 0..100 {
        client.publish(&i.to_string());
    }
    println!("All messages sent");
    thread::sleep(Duration::from_secs(60));
}
