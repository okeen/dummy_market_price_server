use dummy_market_price_server::config_listener::ConfigListener;
use dummy_market_price_server::{DummyMarketPriceProvider, Config, PriceFeedQuote};

use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;


#[tokio::main]
async fn main() {
    setup_tracing();

    let isins = vec!["isin1", "isin2", "isin3"];
    let payloads = isins
        .iter()
        .map(|isin| {
            let quote = PriceFeedQuote::generate_random_price_feed_quote(isin.to_string());
            serde_json::to_string(&quote).unwrap()
        })
        .collect::<Vec<_>>();

    let config = Config {
        server_port: 7000,
        message_limit: None,
        message_delay_millis: Some(100),
        payloads: payloads,    
    }.to_shared();

    let config_listener = ConfigListener::new(Arc::clone(&config));
    let market_price_provider = DummyMarketPriceProvider::new(Arc::clone(&config));

    // Start the config change listener
    tokio::spawn(async move {
        config_listener.listen().await;
    });

    // Start the market price provider
    market_price_provider.run().await;
}

fn setup_tracing() {
    let subscriber = FmtSubscriber::builder()
        // Display the target (module path) of the event
        .with_target(true)
        // Display the source code file and line number of the event
        .with_file(true)
        .with_line_number(true)
        // Set span events to log when spans are created, entered, exited, or closed
        .with_span_events(FmtSpan::FULL)
        // Use RUST_LOG environment variable to set the level filter, defaulting to showing info level
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Tracing initialized.");
}
