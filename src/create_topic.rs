use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::config::ClientConfig;
use rdkafka::error::RDKafkaErrorCode;
use tokio;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let brokers = std::env::var("KAFKA_BROKERS").unwrap_or("localhost:9092".to_string());
    let topic_name = std::env::var("KAFKA_TOPIC").unwrap_or("test".to_string());

    create_kafka_topic(&brokers, &topic_name).await;
}

pub async fn create_kafka_topic(brokers: &str, topic_name: &str) {
    let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()
        .expect("Admin client creation failed");

    let new_topic = NewTopic::new(topic_name, 1, TopicReplication::Fixed(1));

    match admin_client
        .create_topics(&[new_topic], &AdminOptions::new())
        .await
    {
        Ok(responses) => match &responses[0] {
            Ok(_) => println!("✅ Created topic: {}", topic_name),
            Err((_, err)) if err == &RDKafkaErrorCode::TopicAlreadyExists => {
                println!("⚠️ Topic '{}' already exists.", topic_name);
            }
            Err((_, err)) => eprintln!("❌ Failed to create topic '{}': {:?}", topic_name, err),
        },
        Err(e) => eprintln!("❌ Admin request failed: {:?}", e),
    }
}
