use dummy_market_price_server::DummyMarketPriceProvider;
use dummy_market_price_server::config::Config;
use std::sync::Arc;
use tokio::{sync::RwLock, time::Instant};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio::time::{sleep, Duration};
use futures::stream::StreamExt;

#[tokio::test]
async fn test_sends_correct_messages() {
    const MESSAGE_LIMIT: usize = 3;
    const MESSAGE_DELAY: u64 = 20;
    
    let payloads = vec![
        "message 1".to_string(),
        "message 2".to_string(),
        // ... other test messages ...
    ];

    let config = Config {
        payloads: payloads.clone(),
        server_port: 12345, // Ensure this is a unique port for testing
        message_limit: Some(MESSAGE_LIMIT),
        message_delay_millis: Some(MESSAGE_DELAY),
    };

    let arc_config = Arc::new(RwLock::new(config));

    let provider = DummyMarketPriceProvider::new(arc_config.clone());
    tokio::spawn(async move {
        provider.run().await;
    });

    // Give the server some time to start
    sleep(Duration::from_millis(100)).await;

    // Connect to the WebSocket server
    let url = format!("ws://localhost:{}", arc_config.read().await.server_port);
    let (mut ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to WebSocket server");


    let mut total_messages = 0;
    let mut last_received_time = Instant::now();

    for _ in 0..MESSAGE_LIMIT {
        let mut received_messages = Vec::new();

        for _ in 0..payloads.len() {
            if let Some(Ok(Message::Text(msg))) = ws_stream.next().await {
                received_messages.push(msg);
                total_messages += 1;

                // The first message has no delay
                if total_messages == 1 {
                    continue;
                }

                let now = Instant::now();
                let elapsed = now.duration_since(last_received_time);

                assert!(
                    elapsed.as_millis() >= MESSAGE_DELAY as u128,
                    "Message received too early: {elapsed:?}",
                );
    
                last_received_time = now;
            }
        }
        assert_eq!(received_messages, payloads);
    }
    assert_eq!(total_messages, MESSAGE_LIMIT * payloads.len());
}
