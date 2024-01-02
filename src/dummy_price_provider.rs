use rand::Rng;
use serde::Serialize;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use tokio::time::Duration;
use serde_json;
use tracing::{error, info, debug};
use futures::{StreamExt,SinkExt};

#[derive(Debug, Clone, Serialize)]
pub struct PriceFeedQuote {
    pub isin: String,
    pub provider: String,
    pub status: String,
    pub bid: f64,
    pub bid_volume: f64,
    pub ask: f64,
    pub ask_volume: f64,
    pub date_time: String, // Consider using a proper date/time type if needed
}

pub async fn start_dummy_server(isins: Vec<String>) {
    let server = TcpListener::bind("127.0.0.1:7000").await.unwrap();
    info!("Dummy WebSocket server started on ws://127.0.0.1:7000");

    tokio::spawn(async move {
        while let Ok((stream, _)) = server.accept().await {
            let isins = isins.clone();
                if let Ok(ws_stream) = accept_async(stream).await {
                    let (mut write, _) = ws_stream.split();
            
                    loop {
                        for isin in &isins {
                            debug!("Sending quote data...");
                            let quote = generate_random_price_feed_quote(isin.clone());
                            let message = Message::Text(serde_json::to_string(&quote).unwrap());
                            if let Err(e) = write.send(message).await {
                                error!("Error sending message: {:?}", e);
                                break;
                            }
                        }
                        tokio::time::sleep(Duration::from_secs(10)).await;
                    }
                }
        }
    });
    
}
fn generate_random_price_feed_quote(isin: String) -> PriceFeedQuote {
    let mut rng = rand::thread_rng();
    PriceFeedQuote {
        isin: isin,
        provider: "DummyProvider".to_string(),
        status: "Active".to_string(),
        bid: rng.gen_range(100.0..200.0),
        ask: rng.gen_range(100.0..200.0),
        ask_volume: 0.0,
        bid_volume: 0.0,
        date_time: "".to_string(),
    }
}

