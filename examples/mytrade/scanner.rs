// allMids:

//     Subscription message: { "type": "allMids", "dex": "<dex>" }

//     Data format: AllMids 

//     The dex field represents the perp dex to source mids from.

//     Note that the dex field is optional. If not provided, then the first perp dex is used. Spot mids are only included with the first perp dex.

// notification:

//     Subscription message: { "type": "notification", "user": "<address>" }

//     Data format: Notification

// webData3 :

//     Subscription message: { "type": "webData3", "user": "<address>" }

//     Data format: WebData3 

// twapStates :

//     Subscription message: { "type": "twapStates", "user": "<address>", "dex": "<dex>" }

//     Data format: TwapStates 

// clearinghouseState:

//     Subscription message: { "type": "clearinghouseState", "user": "<address>", "dex": "<dex>" }

//     Data format: ClearinghouseState 

// openOrders:

//     Subscription message: { "type": "openOrders", "user": "<address>", "dex": "<dex>" }

//     Data format: OpenOrders 

// candle:

//     Subscription message: { "type": "candle", "coin": "<coin_symbol>", "interval": "<candle_interval>" }

//      Supported intervals: "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "8h", "12h", "1d", "3d", "1w", "1M"

//     Data format: Candle[]

// l2Book:

//     Subscription message: { "type": "l2Book", "coin": "<coin_symbol>" }

//     Optional parameters: nSigFigs: int, mantissa: int

//     Data format: WsBook

// trades:

//     Subscription message: { "type": "trades", "coin": "<coin_symbol>" }

//     Data format: WsTrade[]

// orderUpdates:

//     Subscription message: { "type": "orderUpdates", "user": "<address>" }

//     Data format: WsOrder[]

// userEvents: 

//     Subscription message: { "type": "userEvents", "user": "<address>" }

//     Data format: WsUserEvent

// userFills: 

//     Subscription message: { "type": "userFills", "user": "<address>" }

//     Optional parameter:  aggregateByTime: bool 

//     Data format: WsUserFills

// userFundings: 

//     Subscription message: { "type": "userFundings", "user": "<address>" }

//     Data format: WsUserFundings

// userNonFundingLedgerUpdates: 

//     Subscription message: { "type": "userNonFundingLedgerUpdates", "user": "<address>" }

//     Data format: WsUserNonFundingLedgerUpdates

// activeAssetCtx: 

//     Subscription message: { "type": "activeAssetCtx", "coin": "<coin_symbol>" }

//     Data format: WsActiveAssetCtx or WsActiveSpotAssetCtx 

// activeAssetData: (only supports Perps)

//     Subscription message: { "type": "activeAssetData", "user": "<address>", "coin": "<coin_symbol>" }

//     Data format: WsActiveAssetData

// userTwapSliceFills: 

//     Subscription message: { "type": "userTwapSliceFills", "user": "<address>" }

//     Data format: WsUserTwapSliceFills

// userTwapHistory: 

//     Subscription message: { "type": "userTwapHistory", "user": "<address>" }

//     Data format: WsUserTwapHistory

// bbo :

//     Subscription message: { "type": "bbo", "coin": "<coin>" }

// use alloy::network::AnyHeader;
// use alloy::network::AnyHeader;
//     Data format: WsBbo
use hypersdk::hypercore::{
    self, 
    types::*,
    Trade,
    L2Book,
    OrderUpdate,
    Candle
    
};
use futures::StreamExt;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut ws = hypercore::mainnet_ws();

    // Subscribe to market data
    ws.subscribe(Subscription::Trades { coin: "BTC".into() });
    ws.subscribe(Subscription::L2Book { coin: "ETH".into() });
    ws.subscribe(Subscription::Candle { coin: "BTC".into(), interval: "1m".into()});

    // Process incoming messages
    while let Some(msg) = ws.next().await {
        match msg {
            Incoming::Trades(trades) => {
                on_trades_event(trades).await?;
            }
            Incoming::L2Book(book) => {
                on_l2book_event(book).await?;
            }
            Incoming::SubscriptionResponse(_) => {
                on_subscription_response().await?;
            }
            Incoming::Candle(candle) => {
                on_candle_event(candle).await?;
            }
            Incoming::OrderUpdates(updates) => {
                on_order_update_event(updates).await?;
            }

            _ => {}
        }
    }

    Ok(())
}



async fn on_order_update_event(updates: Vec<OrderUpdate>) -> anyhow::Result<()> {
    println!("Order Update: {:?}", updates);
    Ok(())
}

async fn on_subscription_response() -> anyhow::Result<()> {
    println!("Subscription Confirmed");
    Ok(())
}

async fn on_l2book_event(book: L2Book) -> anyhow::Result<()> {
    println!("Order book update for {:?}", book.coin);
    Ok(())
}

async fn on_trades_event(trades: Vec<Trade>) -> anyhow::Result<()> {
    for trade in trades {
        println!("{} @ {} size {}", trade.side, trade.px, trade.sz);
    }
    Ok(())
}

async fn on_candle_event(candle: Candle) -> anyhow::Result<()> {
    let change = candle.close - candle.open;
    let change_pct = if !candle.open.is_zero() {
        (change / candle.open) * rust_decimal::Decimal::ONE_HUNDRED
    } else {
        rust_decimal::Decimal::ZERO
    };
    let range = candle.high - candle.low;

    // Print formatted candle data
    println!("{} {} candle:", candle.coin, candle.interval);
    println!("  Open:   {}", candle.open);
    println!("  High:   {}", candle.high);
    println!("  Low:    {}", candle.low);
    println!("  Close:  {}", candle.close);
    println!("  Volume: {} {}", candle.volume, candle.coin);
    println!("  Trades: {}", candle.num_trades);
    println!(
        "  Change: {} ({:+.2}%)",
        if change.is_sign_positive() {
            format!("+{}", change)
        } else {
            change.to_string()
        },
        change_pct
    );
    println!("  Range:  {}", range);
    println!("  Time:   {} - {}\n", candle.open_time, candle.close_time);

    Ok(())
}
