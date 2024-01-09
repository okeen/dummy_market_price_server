use tokio::{sync::RwLock, net::TcpListener};
use tokio_tungstenite::tungstenite::Message;
use tracing::info;
use std::sync::Arc;
use crate::config::Config;
use futures::StreamExt;
pub struct ConfigListener {
    config: Arc<RwLock<Config>>,
}

impl ConfigListener {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        Self { config }
    }

    pub async fn listen(&self) {
        let socket = TcpListener::bind("127.0.0.1:7001").await.unwrap();
        info!("Config WebSocket server started on ws://127.0.0.1:7001");

        while let Ok((stream, _)) = socket.accept().await {
            let Ok(ws_stream) = tokio_tungstenite::accept_async(stream).await else { continue };
            info!("Client connected");
        
            let (mut _ws_sender, mut ws_receiver) = ws_stream.split();
            while let Some(message) = ws_receiver.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        info!("Got config message: {text}");
                        // note: this is a hack to remove the slashes from the string. We may need to implement it if the clients can only
                        // send escaped strings instead of plain JSON. 
                        // https://stackoverflow.com/questions/76361360/how-do-i-unescaped-string-that-has-been-escaped-multiple-times-in-rust

                        // let s = stripslashes(&dirty_string).expect("stripslashes failed");
                        // let s: String = serde_json::from_str(&s)?;
                        // let data: Config = serde_json::from_str(&s)?;

                        let Ok(new_config) = serde_json::from_str::<Config>(&text) else {
                            continue;
                        };
                        info!("Parse OK: {new_config:?}");
                        let mut lock = self.config.write().await;
                        *lock = new_config;
                    }
                    // Handle other message types or errors
                    _ => { break; }
                }
            }
        }
    }

}