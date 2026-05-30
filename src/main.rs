use alpaca_api_client::{
    Feed, MarketDataMessage, StockStream,
    market_data::stocks::LatestBarsQuery,
    trading::{
        AccountType,
        order::{CreateOrderQuery, OrderSide, OrderType, TimeInForce},
    },
};

use alpaca_websocket::{AlpacaWebSocketClient, Environment, MarketDataUpdate, SubscribeMessage};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get latest stock prices
    let bars = LatestBarsQuery::new(vec!["AAPL", "GOOGL"])
        .feed("iex") // iex or sip
        .send()?;

    println!("AAPL price: {:?}", bars.get("GOOGL"));

    // StockStream::new(Feed::Iex)
    //     .subscribe_trades(vec!["AAPL", "TSLA"])
    //     .subscribe_bars(vec!["*"])
    //     .start(|msg| match msg {
    //         MarketDataMessage::Trade(trade) => {
    //             println!("Current Price: {}: ${}", trade.symbol, trade.p);
    //         }

    //         MarketDataMessage::Bar(bar) => {
    //             println!("{}: O={} C={}", bar.symbol, bar.o, bar.c);
    //         }
    //         _ => {}
    //     })
    //     .unwrap();

    // // Place a market order
    // let order = CreateOrderQuery::new("AAPL", OrderSide::Buy, OrderType::Stop, TimeInForce::Day)
    //     .qty("1")
    //     .send(AccountType::Paper)?;

    // println!("Order placed: {}", order.id);

    let client = AlpacaWebSocketClient::from_env(Environment::Paper)?;

    let subscription = SubscribeMessage {
        trades: None,
        quotes: Some(vec!["AAPL".to_string()]),
        bars: None,
        trade_updates: None,
    };

    let mut stream = client.subscribe_market_data(subscription).await?;

    while let Some(update) = stream.next().await {
        match update {
            MarketDataUpdate::Bar { symbol, bar } => {
                println!(
                    "Bar: {}: Open: {:?}, High: {:?}, Low: {:?}, Cloe: {:?}, Volume: {:?},",
                    symbol, bar.open, bar.high, bar.low, bar.close, bar.volume
                );
            }

            MarketDataUpdate::Quote { symbol, quote } => {
                println!(
                    "Quote: {}: Bid: {:?}, Ask: {:?}",
                    symbol, quote.bid_price, quote.ask_price
                );
            }

            _ => {}
        }
    }

    Ok(())
}
