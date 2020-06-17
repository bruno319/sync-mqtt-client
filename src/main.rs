use crate::client::MqttClient;
use std::thread;
use std::time::Duration;
use crate::queue::PersistenceQueue;

mod client;
mod queue;

fn main() {
    let client = MqttClient::new(Box::new(PersistenceQueue::new())).unwrap();
    for i in 0..1000 {
        client.publish(&i.to_string());
    }
    println!("All messages sent");
    thread::sleep(Duration::from_secs(60));
}
