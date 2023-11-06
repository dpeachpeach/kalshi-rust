use super::Kalshi;
use serde::{Deserialize, Serialize};

impl<'a> Kalshi<'a> {
    pub async fn get_single_event(
        &self,
        event_ticker: &String,
        with_nested_markets: Option<bool>,
    ) -> Result<Event, reqwest::Error> {
        let single_event_url: &str =
            &format!("{}/events/{}", self.base_url.to_string(), event_ticker);

        let mut params: Vec<(&str, String)> = Vec::with_capacity(2);

        add_param!(params, "with_nested_markets", with_nested_markets);

        let single_event_url = reqwest::Url::parse_with_params(single_event_url, &params)
            .unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                panic!("Internal Parse Error, please contact developer!");
            });

        let result: SingleEventResponse = self
            .client
            .get(single_event_url)
            .send()
            .await?
            .json()
            .await?;

        return Ok(result.event);
    }

    pub async fn get_single_market(&self, ticker: &String) -> Result<Market, reqwest::Error> {
        let single_market_url: &str = &format!("{}/markets/{}", self.base_url.to_string(), ticker);

        let result: SingleMarketResponse = self
            .client
            .get(single_market_url)
            .send()
            .await?
            .json()
            .await?;

        return Ok(result.market);
    }
    pub async fn get_multiple_markets(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        event_ticker: Option<String>,
        series_ticker: Option<String>,
        max_close_ts: Option<i64>,
        min_close_ts: Option<i64>,
        status: Option<String>,
        tickers: Option<String>,
    ) -> Result<(Option<String>, Vec<Market>), reqwest::Error> {
        let markets_url: &str = &format!("{}/markets", self.base_url.to_string());

        let mut params: Vec<(&str, String)> = Vec::with_capacity(10);

        add_param!(params, "limit", limit);
        add_param!(params, "event_ticker", event_ticker);
        add_param!(params, "series_ticker", series_ticker);
        add_param!(params, "status", status);
        add_param!(params, "cursor", cursor);
        add_param!(params, "min_close_ts", min_close_ts);
        add_param!(params, "max_close_ts", max_close_ts);
        add_param!(params, "tickers", tickers);

        let markets_url =
            reqwest::Url::parse_with_params(markets_url, &params).unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                panic!("Internal Parse Error, please contact developer!");
            });

        let result: PublicMarketsResponse = self
            .client
            .get(markets_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .send()
            .await?
            .json()
            .await?;

        Ok((result.cursor, result.markets))
    }

    pub async fn get_multiple_events(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        status: Option<String>,
        series_ticker: Option<String>,
        with_nested_markets: Option<bool>,
    ) -> Result<(Option<String>, Vec<Event>), reqwest::Error> {
        let events_url: &str = &format!("{}/events", self.base_url.to_string());

        let mut params: Vec<(&str, String)> = Vec::with_capacity(6);

        add_param!(params, "limit", limit);
        add_param!(params, "status", status);
        add_param!(params, "cursor", cursor);
        add_param!(params, "series_ticker", series_ticker);
        add_param!(params, "with_nested_markets", with_nested_markets);

        let events_url =
            reqwest::Url::parse_with_params(events_url, &params).unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                panic!("Internal Parse Error, please contact developer!");
            });

        let result: PublicEventsResponse = self.client.get(events_url).send().await?.json().await?;

        return Ok((result.cursor, result.events));
    }

    pub async fn get_series(&self, ticker: &String) -> Result<Series, reqwest::Error> {
        let series_url: &str = &format!("{}/series/{}", self.base_url.to_string(), ticker);

        let result: SeriesResponse = self.client.get(series_url).send().await?.json().await?;

        return Ok(result.series);
    }

    pub async fn get_market_orderbook(
        &self,
        ticker: &String,
        depth: Option<i32>,
    ) -> Result<Orderbook, reqwest::Error> {
        let orderbook_url: &str =
            &format!("{}/markets/{}/orderbook", self.base_url.to_string(), ticker);

        let mut params: Vec<(&str, String)> = Vec::new();

        add_param!(params, "depth", depth);

        let orderbook_url =
            reqwest::Url::parse_with_params(orderbook_url, &params).unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                panic!("Internal Parse Error, please contact developer!");
            });

        let result: OrderBookResponse = self
            .client
            .get(orderbook_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .send()
            .await?
            .json()
            .await?;

        return Ok(result.orderbook);
    }

    pub async fn get_market_history(
        &self,
        ticker: &String,
        limit: Option<i32>,
        cursor: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
    ) -> Result<(Option<String>, Vec<Snapshot>), reqwest::Error> {
        let market_history_url: &str =
            &format! {"{}/markets/{}/history", self.base_url.to_string(), ticker};

        let mut params: Vec<(&str, String)> = Vec::with_capacity(5);

        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "min_ts", min_ts);
        add_param!(params, "max_ts", max_ts);

        let market_history_url = reqwest::Url::parse_with_params(market_history_url, &params)
            .unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                panic!("Internal Parse Error, please contact developer!");
            });

        let result: MarketHistoryResponse = self
            .client
            .get(market_history_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .send()
            .await?
            .json()
            .await?;

        Ok((result.cursor, result.history))
    }

    pub async fn get_trades(
        &self,
        cursor: Option<String>,
        limit: Option<i32>,
        ticker: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
    ) -> Result<(Option<String>, Vec<Trade>), reqwest::Error> {
        let trades_url: &str = &format!("{}/markets/trades", self.base_url.to_string());

        let mut params: Vec<(&str, String)> = Vec::with_capacity(7);

        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "min_ts", min_ts);
        add_param!(params, "max_ts", max_ts);
        add_param!(params, "ticker", ticker);

        let trades_url =
            reqwest::Url::parse_with_params(trades_url, &params).unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                panic!("Internal Parse Error, please contact developer!");
            });

        let result: PublicTradesResponse = self.client.get(trades_url).send().await?.json().await?;

        Ok((result.cursor, result.trades))
    }


}


// PRIVATE STRUCTS 
// used in get_single_event
#[derive(Debug, Deserialize, Serialize)]
struct SingleEventResponse {
    event: Event,
    markets: Option<Vec<Market>>,
}


// used in get_single_market
#[derive(Debug, Deserialize, Serialize)]
struct SingleMarketResponse {
    market: Market,
}

#[derive(Debug, Deserialize, Serialize)]
struct PublicMarketsResponse {
    cursor: Option<String>,
    markets: Vec<Market>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PublicEventsResponse {
    cursor: Option<String>,
    events: Vec<Event>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SeriesResponse {
    series: Series,
}

#[derive(Debug, Deserialize, Serialize)]
struct OrderBookResponse {
    orderbook: Orderbook,
}

#[derive(Debug, Deserialize, Serialize)]
struct MarketHistoryResponse {
    cursor: Option<String>,
    ticker: String,
    history: Vec<Snapshot>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PublicTradesResponse {
    cursor: Option<String>,
    trades: Vec<Trade>,
}


// PUBLIC STRUCTS

// used in get_market and get_markets and get_events method
#[derive(Debug, Deserialize, Serialize)]
pub struct Market {
    pub ticker: String,
    pub event_ticker: String,
    pub market_type: String,
    pub title: String,
    pub subtitle: String,
    pub yes_sub_title: String,
    pub no_sub_title: String,
    pub open_time: String,
    pub close_time: String,
    pub expected_expiration_time: Option<String>,
    pub expiration_time: Option<String>,
    pub latest_expiration_time: String,
    pub settlement_timer_seconds: i64,
    pub status: String,
    pub response_price_units: String,
    pub notional_value: i64,
    pub tick_size: i64,
    pub yes_bid: i64,
    pub yes_ask: i64,
    pub no_bid: i64,
    pub no_ask: i64,
    pub last_price: i64,
    pub previous_yes_bid: i64,
    pub previous_yes_ask: i64,
    pub previous_price: i64,
    pub volume: i64,
    pub volume_24h: i64,
    pub liquidity: i64,
    pub open_interest: i64,
    pub result: SettlementResult,
    pub cap_strike: Option<f64>,
    pub can_close_early: bool,
    pub expiration_value: String,
    pub category: String,
    pub risk_limit_cents: i64,
    pub strike_type: Option<String>,
    pub floor_strike: Option<f64>,
    pub rules_primary: String,
    pub rules_secondary: String,
    pub settlement_value: Option<String>,
    pub functional_strike: Option<String>,
}


// Used in get_event and get_events methods
#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub event_ticker: String,
    pub series_ticker: String,
    pub sub_title: String,
    pub title: String,
    pub mutually_exclusive: bool,
    pub category: String,
    pub markets: Option<Vec<Market>>,
    pub strike_date: Option<String>,
    pub strike_period: Option<String>,
}

// used in get_series
#[derive(Debug, Deserialize, Serialize)]
pub struct Series {
    pub ticker: String,
    pub frequency: String,
    pub title: String,
    pub category:String,
    pub tags: Vec<String>,
    pub settlement_sources: Vec<SettlementSource>,
    pub contract_url: String,
}

// used in get_series
#[derive(Debug, Deserialize, Serialize)]
pub struct SettlementSource {
    pub url: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Orderbook {
    pub yes: Vec<Vec<i32>>,
    pub no: Vec<Vec<i32>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Snapshot {
    pub yes_price: i32,
    pub yes_bid: i32,
    pub yes_ask: i32,
    pub no_bid: i32,
    pub no_ask: i32,
    pub volume: i32,
    pub open_interest: i32,
    pub ts: i64,
}

// used in get_user_fills and get_fill
#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    pub trade_id: String,
    pub taker_side: String,
    pub ticker: String,
    pub count: i32,
    pub yes_price: i32,
    pub no_price: i32,
    pub created_time: String,
}

// ENUMS


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SettlementResult {
    Yes,
    No,
    #[serde(rename = "")]
    Void,
    #[serde(rename = "all_no")]
    AllNo,
    #[serde(rename = "all_yes")]
    AllYes,
}