use serde::Deserialize;

use super::KalshiChannel;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum KalshiWebsocketResponse {
    OrderbookSnapshot {
        sid: u32,
        seq: u32,
        msg: KalshiOrderbookSnapshotMessage,
    },
    OrderbookDelta {
        sid: u32,
        seq: u32,
        msg: KalshiOrderbookDeltaMessage,
    },
    Ticker {
        sid: u32,
        msg: KalshiTickerMessage,
    },
    Trade {
        sid: u32,
        msg: KalshiTradeMessage,
    },
    Fill {
        sid: u32,
        msg: KalshiTickerMessage,
    },
    MarketLifecycle {
        sid: u32,
        msg: KalshiMarketLifecycleMessage,
    },
    Subscribed {
        msg: KalshiOrderbookSubscribedMessage,
    },
    Error {
        id: u32,
        msg: KalshiOrderbookErrorMessage,
    },
    Ok {
        id: u32,
        sid: u32,
        seq: u32,
        market_tickers: Vec<String>,
    },
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiOrderbookSubscribedMessage {
    channel: KalshiChannel,
    sid: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiOrderbookErrorMessage {
    code: u32,
    msg: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiOrderbookSnapshotMessage {
    market_ticker: String,
    yes: Option<Vec<(u32, i32)>>,
    no: Option<Vec<(u32, i32)>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiOrderbookDeltaMessage {
    delta: i32,
    price: u32,
    side: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiTickerMessage {
    market_ticker: String,
    price: u32,
    yes_bid: u32,
    yes_ask: u32,
    volume: u32,
    open_interest: u32,
    dollar_volume: u32,
    dollar_open_interest: u32,
    ts: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiTradeMessage {
    market_ticker: String,
    yes_price: u32,
    no_price: u32,
    count: u32,
    taker_side: KalshiSide,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiFillMessage {
    trade_id: String,
    order_id: String,
    market_ticker: String,
    is_taker: bool,
    side: KalshiSide,
    yes_price: u32,
    no_price: u32,
    count: u32,
    ts: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMarketLifecycleMessage {
    market_ticker: String,
    close_ts: u32,
    determination_ts: Option<u32>,
    settled_ts: Option<u32>,
    result: Option<String>,
    is_deactivated: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KalshiSide {
    Yes,
    No,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KalshiAction {
    Buy,
    Sell,
}
