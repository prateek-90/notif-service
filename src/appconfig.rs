use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    pub server: ServerConfig,
    // kafka: KafkaConfig, 
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ServerConfig {
    pub context_path: String,
    pub port: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct KafkaTopicConfig {
    broker: String,
    topic: String,
    zookeeper: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct KafkaConfig {
    topic_config: HashMap<String, KafkaTopicConfig>,
}