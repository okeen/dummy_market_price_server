use serde::Deserialize;

extern crate dotenv;
use dotenv::dotenv;
use tokio::net::TcpListener;
use rand::Rng;
use serde::Serialize;
use tokio_tungstenite::{tungstenite::protocol::Message};
use tokio::time::Duration;
use serde_json;
use tracing::{error, info, debug};
use futures::{StreamExt,SinkExt};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub dummy_security_prices_isins: String,
}

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

#[tokio::main]
async fn main() {
    use tracing_subscriber::{fmt::format::FmtSpan};
    tracing_subscriber::fmt()
        // Filter what traces are displayed based on the RUST_LOG environment
        // variable.
        //
        // Traces emitted by the example code will always be displayed. You
        // can set `RUST_LOG=tokio=trace` to enable additional traces emitted by
        // Tokio itself.
        // .with_env_filter(EnvFilter::from_default_env().add_directive("chat=info".parse()?))
        // Log events when `tracing` spans are created, entered, exited, or
        // closed. When Tokio's internal tracing support is enabled (as
        // described above), this can be used to track the lifecycle of spawned
        // tasks on the Tokio runtime.
        .with_span_events(FmtSpan::FULL)
        // Set this subscriber as the default, to collect all traces emitted by
        // the program.
        .init();
    dotenv().ok();
    
    let config: Config = envy::from_env().unwrap();
    let isins: Vec<String> = config.dummy_security_prices_isins.split(",").map(String::from).collect();
    
    let server = TcpListener::bind("127.0.0.1:7000").await.unwrap();
    info!("Dummy WebSocket server started on ws://127.0.0.1:7000");

    loop {
        while let Ok((stream, _)) = server.accept().await {
            let isins = isins.clone();
        
            tokio::spawn(async move {
                debug!("Client connected");
                if let Ok(ws_stream) = tokio_tungstenite::accept_async(stream).await {
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
                        tokio::time::sleep(Duration::from_millis(1)).await;
                    }
                }
            });
        
        }
    }
    
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


