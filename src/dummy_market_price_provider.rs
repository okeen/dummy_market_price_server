use crate::config::Config;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::time::{Duration, sleep};
use tokio_tungstenite::tungstenite::protocol::Message;
use tracing::{error, info, debug};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;

pub struct DummyMarketPriceProvider {
    config: Arc<RwLock<Config>>,
}

impl DummyMarketPriceProvider {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        Self { config }
    }

    pub async fn run(&self) {
        let server = TcpListener::bind(format!("0.0.0.0:{}", self.config.read().await.server_port))
            .await
            .unwrap();
        info!("Dummy WebSocket server started on ws://0.0.0.0:{}", self.config.read().await.server_port);

        loop {
            while let Ok((stream, _)) = server.accept().await {
                info!("New connection established");

                let task_config = self.config.read().await;
                info!("Config: {:?}", task_config);

                let delay = Duration::from_millis(task_config.message_delay_millis.unwrap_or(0));
                let payloads = task_config.payloads.clone();
                let message_limit = task_config.message_limit;
                let message_count = message_limit.unwrap_or(std::usize::MAX);

                tokio::spawn(async move {
                    if let Ok(ws_stream) = tokio_tungstenite::accept_async(stream).await {
                        let (mut write, _) = ws_stream.split();

                        for _ in 0..message_count {
                            for payload in &payloads {
                                debug!("Sending quote data...");
                                let message = Message::Text(payload.to_string());

                                if write.send(message).await.is_err() {
                                    error!("Error sending message");
                                    return;
                                }

                                sleep(delay).await;
                            }
                        }
                    }
                });
            }
        }
    }
}

