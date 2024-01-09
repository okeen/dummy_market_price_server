use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceFeedQuote {
    pub isin: String,
    pub provider: String,
    pub status: String,
    pub bid: f64,
    pub bid_volume: f64,
    pub ask: f64,
    pub ask_volume: f64,
    pub date_time: String,
}

impl PriceFeedQuote {
    pub fn generate_random_price_feed_quote(isin: String) -> PriceFeedQuote {
        let mut rng = rand::thread_rng();
    
        PriceFeedQuote {
            isin,
            provider: "DummyProvider".to_string(),
            status: "Active".to_string(),
            bid: rng.gen_range(100.0..200.0),
            ask: rng.gen_range(100.0..200.0),
            ask_volume: 0.0,
            bid_volume: 0.0,
            date_time: "".to_string(),
        }
    }
}