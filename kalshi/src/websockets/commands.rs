use super::KalshiChannel;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "cmd")]
#[serde(rename_all = "snake_case")]
pub enum KalshiCommand {
    Subscribe {
        id: u32,
        params: KalshiSubscribeCommandParams,
    },
    UpdateSubscription {
        id: u32,
        params: KalshiUpdateSubscriptionCommandParams,
    },
    Unsubscribe {
        id: u32,
        params: KalshiUnsubscribeCommandParams,
    },
    // UpdateSubscription
    // Unsubscribe
    End, // Not a Kalshi command but signals to send prob messaging to close the ws stream
}
#[derive(Serialize, Clone, Debug)]
pub struct KalshiSubscribeCommandParams {
    pub channels: Vec<KalshiChannel>,
    pub market_tickers: Vec<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct KalshiUnsubscribeCommandParams {
    pub sids: Vec<u32>,
}

#[derive(Serialize, Clone, Debug)]
pub struct KalshiUpdateSubscriptionCommandParams {
    pub action: KalshiUpdateSubscriptionAction,
    pub market_tickers: Vec<String>,
    pub sids: [u32; 1],
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum KalshiUpdateSubscriptionAction {
    AddMarkets,
    DeleteMarkets,
}
