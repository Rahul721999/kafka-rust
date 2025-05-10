use dotenv;
use tokio;

use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::config::ClientConfig;
use rdkafka::error::RDKafkaErrorCode;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let brokers = std::env::var("KAFKA_BROKERS").expect("KAFKA_BROKERS must be set");
    let topic_name = std::env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC must be set");

    // Create AdminClient
    let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", &brokers)
        .create()
        .expect("Admin client creation failed");

    // Describe topics to check existence
    let metadata = admin_client
        .inner()
        .fetch_metadata(Some(&topic_name), std::time::Duration::from_secs(5))
        .unwrap();

    let topic_exists = metadata.topics().iter().any(|t| t.name() == topic_name);

    if topic_exists {
        println!("✅ Topic '{}' already exists.", topic_name);
    } else {
        println!("⚙️ Topic '{}' not found. Creating...", topic_name);

        // Create new topic
        let new_topic = NewTopic::new(
            &topic_name,
            1,                     // partitions
            TopicReplication::Fixed(1), // replication factor
        );

        match admin_client
            .create_topics(&[new_topic], &AdminOptions::new())
            .await
        {
            Ok(responses) => {
                for response in responses {
                    match response {
                        Ok(topic) => println!("✅ Created topic: {}", topic),
                        Err((topic, err)) => {
                            if err.eq(&RDKafkaErrorCode::TopicAlreadyExists) {
                                println!("⚠️ Topic '{}' already exists.", topic);
                            } else {
                                eprintln!("❌ Failed to create topic '{}': {:?}", topic, err);
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("❌ Admin request failed: {:?}", e),
        }
    }
}
