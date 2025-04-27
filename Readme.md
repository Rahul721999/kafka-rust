# Kafka-Rust

This repository is dedicated to self-learning Kafka using Rust.

## Quick Start

1. **Run Kafka with Docker**:

    ```bash
    docker run -d --name kafka-broker apache/kafka:latest
    ```

2. **Access Broker Shell**:

    ```bash
    docker exec -it kafka-broker sh
    ```

3. **Create a Kafka Topic**:

    ```bash
    ./kafka-topics.sh --bootstrap-server localhost:9092 --create --topic test-topic
    ```

4. **Start a Producer**:

    ```bash
    ./kafka-console-producer.sh --bootstrap-server localhost:9092 --topic test-topic
    ```

5. **Start a Consumer**:

    ```bash
    ./kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic test-topic --from-beginning
    ```

6. **Cleanup**:

    ```bash
    docker rm -f kafka-broker
    ```
