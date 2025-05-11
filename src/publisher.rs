use rand::rng;
use rand::seq::IndexedRandom;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::{json, Value};
use std::time::Duration as StdDuration;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let brokers = std::env::var("KAFKA_BROKERS").unwrap_or("localhost:9092".to_string());
    let topic = std::env::var("KAFKA_TOPIC").unwrap_or("tasks".to_string());

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &brokers)
        .create()
        .expect("Producer creation error");

    let task_types = vec!["email", "sms", "push"];

    loop {
        let task_type = task_types.choose(&mut rng()).unwrap();
        println!("📧 Sending task: {:?}", task_type);
        let payload = match *task_type {
            "email" => json!({
                "to": "user@example.com",
                "subject": "Hello",
                "body": "Hi there!"
            }),
            "sms" => json!({
                "to": "+1234567890",
                "message": "Your code is 1234"
            }),
            "push" => json!({
                "device_id": "abc123",
                "message": "New update available!"
            }),
            _ => json!({}),
        };

        let task = json!({
            "type": task_type,
            "payload": payload
        });

        send(&producer, task, &topic).await;
        sleep(Duration::from_secs(5)).await;
    }
}

async fn send(producer: &FutureProducer, task: Value, topic: &str) {
    let task = serde_json::to_string(&task).unwrap();
    let record: FutureRecord<'_, (), [u8]> = FutureRecord::to(&topic).payload(task.as_bytes());

    match producer.send(record, StdDuration::from_secs(0)).await {
        Ok(delivery) => println!("✅ Sent: {:?}", delivery),
        Err((err, _)) => eprintln!("❌ Failed to send: {:?}", err),
    }
}