use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;
use rdkafka::Message;
use serde_json::Value;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let brokers = std::env::var("KAFKA_BROKERS").unwrap_or("localhost:9092".to_string());
    let group_id = "task_group";
    let topic = std::env::var("KAFKA_TOPIC").unwrap_or("tasks".to_string());

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", &brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&[&topic]).expect("Subscription failed");

    println!("🚀 Listening for messages on topic '{}'", topic);

    let mut message_stream = consumer.stream();

    while let Ok(message_result) = message_stream.try_next().await {
        match message_result {
            Some(message) => handle_message(&consumer, &message).await,
            None => eprintln!("❌ Error while reading from stream"),
        }
    }
}

async fn handle_message<C: Consumer>(consumer: &C, msg: &BorrowedMessage<'_>) {
    if let Some(payload) = msg.payload_view::<str>(){
        let payload = payload.unwrap();
        match serde_json::from_str::<Value>(payload) {
            Ok(json) => {
                if let Some(task_type) = json.get("type").and_then(|v| v.as_str()) {
                    match task_type {
                        "email" => handle_email(&json).await,
                        "sms" => handle_sms(&json).await,
                        "push" => handle_push(&json).await,
                        _ => println!("⚠️ Unknown task type: {}", task_type),
                    }
                } else {
                    println!("⚠️ Missing 'type' field in message.");
                }
            }
            Err(e) => eprintln!("❌ Failed to parse JSON: {:?}", e),
        }
    }

    // Commit message offset
    if let Err(e) = consumer.commit_message(msg, CommitMode::Async) {
        eprintln!("❌ Failed to commit message: {:?}", e);
    }
}

async fn handle_email(json: &Value) {
    println!("📧 Handling email task: {:?}", json);
}

async fn handle_sms(json: &Value) {
    println!("📱 Handling SMS task: {:?}", json);
}

async fn handle_push(json: &Value) {
    println!("🔔 Handling push notification task: {:?}", json);
}