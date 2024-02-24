pub mod binance_data{
    use barter_data::{
        exchange::binance::spot::BinanceSpot,
        streams::Streams,
        subscription::trade::PublicTrades,
    };
    
    use barter_integration::model::instrument::kind::InstrumentKind;
    use barter_data::event::MarketEvent;
    use barter_data::subscription::trade::PublicTrade;
    
    
    // Tokio main function
    #[tokio::main]
    pub async fn main() -> Streams<MarketEvent<PublicTrade>>{
        // Create Streams builder
        let streams = Streams::<PublicTrades>::builder()
            .subscribe([
                (
                    BinanceSpot::default(),
                    "btc",
                    "usdt",
                    InstrumentKind::Spot,
                    PublicTrades,
                ),
                (
                    BinanceSpot::default(),
                    "eth",
                    "usdt",
                    InstrumentKind::Spot,
                    PublicTrades,
                ),
            ])
            .init()
            .await
            .unwrap(); 

        streams
    }
}