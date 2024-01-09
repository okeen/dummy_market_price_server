use std::sync::{Arc};
use tokio::sync::RwLock;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub payloads: Vec<String>,
    pub server_port: u16,
    pub message_limit: Option<usize>,
    pub message_delay_millis: Option<u64>,
}

type SharedConfig = Arc<RwLock<Config>>;

impl Config {
    pub fn to_shared(&self) -> SharedConfig {
        let config = self.clone();
        Arc::new(RwLock::new(config))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            payloads: vec![],
            server_port: 7000,
            message_limit: None,
            message_delay_millis: None,
        }
    }
}