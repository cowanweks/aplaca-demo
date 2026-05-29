use alpaca_api_client::{
    Feed, MarketDataMessage, StockStream,
    market_data::stocks::LatestBarsQuery,
    trading::{
        AccountType,
        order::{CreateOrderQuery, OrderSide, OrderType, TimeInForce},
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get latest stock prices
    let bars = LatestBarsQuery::new(vec!["AAPL", "GOOGL"])
        .feed("sip")
        .send()?;

    println!("AAPL price: {:?}", bars.get("GOOGL"));

    StockStream::new(Feed::Iex)
        .subscribe_trades(vec!["AAPL", "TSLA"])
        .subscribe_bars(vec!["*"])
        .start(|msg| match msg {
            MarketDataMessage::Trade(trade) => {
                println!("{}: ${}", trade.symbol, trade.p);
            }
            MarketDataMessage::Bar(bar) => {
                println!("{}: O={} C={}", bar.symbol, bar.o, bar.c);
            }
            _ => {}
        })
        .unwrap();

    // Place a market order
    let order = CreateOrderQuery::new("AAPL", OrderSide::Buy, OrderType::Stop, TimeInForce::Day)
        .qty("1")
        .send(AccountType::Paper)?;

    println!("Order placed: {}", order.id);

    Ok(())
}
