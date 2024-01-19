use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves detailed information about a specific event from the Kalshi exchange.
    ///
    /// # Arguments
    /// * `event_ticker` - A string reference representing the ticker of the event.
    /// * `with_nested_markets` - An optional boolean to include nested market data.
    ///
    /// # Returns
    /// - `Ok(Event)`: Event object on successful retrieval.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    /// # Example
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let event_ticker = "some_event_ticker";
    /// let event = kalshi_instance.get_single_event(event_ticker, None).await.unwrap();
    /// ```
    pub async fn get_single_event(
        &self,
        event_ticker: &String,
        with_nested_markets: Option<bool>,
    ) -> Result<Event, KalshiError> {
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

    /// Retrieves detailed information about a specific market from the Kalshi exchange.
    ///
    /// # Arguments
    /// * `ticker` - A string reference representing the ticker of the market.
    ///
    /// # Returns
    /// - `Ok(Market)`: Market object on successful retrieval.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    /// # Example
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let market_ticker = "some_event_ticker";
    /// let market = kalshi_instance.get_single_event(market_ticker).await.unwrap();
    /// ```
    pub async fn get_single_market(&self, ticker: &String) -> Result<Market, KalshiError> {
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
    /// Asynchronously retrieves information about multiple markets from the Kalshi exchange.
    ///
    /// This method fetches data for a collection of markets, filtered by various optional parameters.
    /// It supports pagination, time-based filtering, and selection by specific tickers or statuses.
    ///
    /// # Arguments
    /// * `limit` - An optional integer to limit the number of markets returned.
    /// * `cursor` - An optional string for pagination cursor.
    /// * `event_ticker` - An optional string to filter markets by event ticker.
    /// * `series_ticker` - An optional string to filter markets by series ticker.
    /// * `max_close_ts` - An optional timestamp for the maximum close time.
    /// * `min_close_ts` - An optional timestamp for the minimum close time.
    /// * `status` - An optional string to filter markets by their status.
    /// * `tickers` - An optional string to filter markets by specific tickers.
    ///
    /// # Returns
    /// - `Ok((Option<String>, Vec<Market>))`: A tuple containing an optional pagination cursor and a vector of `Market` objects on success.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let markets_result = kalshi_instance.get_multiple_markets(
    ///     Some(10),
    ///     None,
    ///     Some("event_ticker"),
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     None
    /// ).await.unwrap();
    /// ```
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
    ) -> Result<(Option<String>, Vec<Market>), KalshiError> {
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
    /// Asynchronously retrieves information about multiple events from the Kalshi exchange.
    ///
    /// This method fetches data for multiple events, with optional filtering based on status,
    /// series ticker, and whether nested market data should be included. It supports pagination
    /// and time-based filtering.
    ///
    /// # Arguments
    /// * `limit` - An optional integer to limit the number of events returned.
    /// * `cursor` - An optional string for pagination cursor.
    /// * `status` - An optional string to filter events by their status.
    /// * `series_ticker` - An optional string to filter events by series ticker.
    /// * `with_nested_markets` - An optional boolean to include nested market data.
    ///
    /// # Returns
    /// - `Ok((Option<String>, Vec<Event>))`: A tuple containing an optional pagination cursor and a vector of `Event` objects on success.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let events_result = kalshi_instance.get_multiple_events(
    ///     Some(10),
    ///     None,
    ///     Some("active"),
    ///     None,
    ///     Some(true)
    /// ).await.unwrap();
    /// println!("Events: {:?}", events_result);
    /// ```
    ///
    pub async fn get_multiple_events(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        status: Option<String>,
        series_ticker: Option<String>,
        with_nested_markets: Option<bool>,
    ) -> Result<(Option<String>, Vec<Event>), KalshiError> {
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
    /// Asynchronously retrieves detailed information about a specific series from the Kalshi exchange.
    ///
    /// This method fetches data for a series identified by its ticker. The series data includes
    /// information such as frequency, title, category, settlement sources, and related contract URLs.
    ///
    /// # Arguments
    /// * `ticker` - A reference to a string representing the series's ticker.
    ///
    /// # Returns
    /// - `Ok(Series)`: `Series` object on successful retrieval.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    /// # Example
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let series_ticker = "some_series_ticker";
    /// let series = kalshi_instance.get_series(series_ticker).await.unwrap();
    /// ```
    pub async fn get_series(&self, ticker: &String) -> Result<Series, KalshiError> {
        let series_url: &str = &format!("{}/series/{}", self.base_url.to_string(), ticker);

        let result: SeriesResponse = self.client.get(series_url).send().await?.json().await?;

        return Ok(result.series);
    }
    /// Asynchronously retrieves the order book for a specific market in the Kalshi exchange.
    ///
    /// This method fetches the order book for a market, which includes the bid and ask prices
    /// for both 'Yes' and 'No' options. It allows specifying the depth of the order book to be retrieved.
    ///
    /// # Arguments
    /// * `ticker` - A reference to a string representing the market's ticker.
    /// * `depth` - An optional integer specifying the depth of the order book.
    ///
    /// # Returns
    /// - `Ok(Orderbook)`: `Orderbook` object on successful retrieval.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    ///
    /// # Example
    /// Returns an orderbook with a depth of 10 entries for some market.
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let market_ticker = "some_market_ticker";
    /// let orderbook = kalshi_instance.get_market_orderbook(market_ticker, Some(10)).await.unwrap();
    /// ```
    pub async fn get_market_orderbook(
        &self,
        ticker: &String,
        depth: Option<i32>,
    ) -> Result<Orderbook, KalshiError> {
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

    /// Asynchronously retrieves the market history for a given market on the Kalshi exchange.
    ///
    /// This method fetches historical data for a specific market, which can include
    /// details like prices, bids, asks, volume, and open interest over time. It allows
    /// filtering the history based on time and pagination parameters.
    ///
    /// # Arguments
    /// * `ticker` - A reference to a string representing the market's ticker.
    /// * `limit` - An optional integer to limit the number of history records returned.
    /// * `cursor` - An optional string for pagination cursor.
    /// * `min_ts` - An optional timestamp to specify the minimum time for history records.
    /// * `max_ts` - An optional timestamp to specify the maximum time for history records.
    ///
    /// # Returns
    /// - `Ok((Option<String>, Vec<Snapshot>))`: A tuple containing an optional pagination cursor and a vector of `Snapshot` objects on success.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let market_history = kalshi_instance.get_market_history(
    ///     "ticker_name",
    ///     Some(10),
    ///     None,
    ///     None,
    ///     None
    /// ).await.unwrap();
    /// ```
    pub async fn get_market_history(
        &self,
        ticker: &String,
        limit: Option<i32>,
        cursor: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
    ) -> Result<(Option<String>, Vec<Snapshot>), KalshiError> {
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

    /// Asynchronously retrieves trade data from the Kalshi exchange.
    ///
    /// This method fetches data about trades that have occurred, including details like trade ID,
    /// taker side, ticker, and executed prices. It supports filtering based on various parameters
    /// such as time, ticker, and pagination options.
    ///
    /// # Arguments
    /// * `cursor` - An optional string for pagination cursor.
    /// * `limit` - An optional integer to limit the number of trades returned.
    /// * `ticker` - An optional string representing the market's ticker for which trades are to be fetched.
    /// * `min_ts` - An optional timestamp to specify the minimum time for trade records.
    /// * `max_ts` - An optional timestamp to specify the maximum time for trade records.
    ///
    /// # Returns
    /// - `Ok((Option<String>, Vec<Trade>))`: A tuple containing an optional pagination cursor and a vector of `Trade` objects on success.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let trades = kalshi_instance.get_trades(
    ///     None,
    ///     Some(10),
    ///     Some("ticker_name"),
    ///     None,
    ///     None
    /// ).await.unwrap();
    /// ```
    pub async fn get_trades(
        &self,
        cursor: Option<String>,
        limit: Option<i32>,
        ticker: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
    ) -> Result<(Option<String>, Vec<Trade>), KalshiError> {
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

/// A market in the Kalshi exchange.
///
/// Contains detailed information about the market including its ticker,
/// type, status, and other relevant data.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Market {
    /// Unique identifier for the market.
    pub ticker: String,
    /// Ticker of the associated event.
    pub event_ticker: String,
    /// Type of the market.
    pub market_type: String,
    /// Title of the market.
    pub title: String,
    /// Subtitle of the market.
    pub subtitle: String,
    /// Subtitle for the 'Yes' option in the market.
    pub yes_sub_title: String,
    /// Subtitle for the 'No' option in the market.
    pub no_sub_title: String,
    /// Opening time of the market.
    pub open_time: String,
    /// Closing time of the market.
    pub close_time: String,
    /// Expected expiration time of the market.
    pub expected_expiration_time: Option<String>,
    /// Actual expiration time of the market.
    pub expiration_time: Option<String>,
    /// Latest possible expiration time of the market.
    pub latest_expiration_time: String,
    /// Countdown in seconds to the settlement.
    pub settlement_timer_seconds: i64,
    /// Current status of the market.
    pub status: String,
    /// Units used for pricing responses.
    pub response_price_units: String,
    /// Notional value of the market.
    pub notional_value: i64,
    /// Minimum price movement in the market.
    pub tick_size: i64,
    /// Current bid price for the 'Yes' option.
    pub yes_bid: i64,
    /// Current ask price for the 'Yes' option.
    pub yes_ask: i64,
    /// Current bid price for the 'No' option.
    pub no_bid: i64,
    /// Current ask price for the 'No' option.
    pub no_ask: i64,
    /// Last traded price in the market.
    pub last_price: i64,
    /// Previous bid price for the 'Yes' option.
    pub previous_yes_bid: i64,
    /// Previous ask price for the 'Yes' option.
    pub previous_yes_ask: i64,
    /// Previous traded price in the market.
    pub previous_price: i64,
    /// Total trading volume in the market.
    pub volume: i64,
    /// Trading volume in the last 24 hours.
    pub volume_24h: i64,
    /// Liquidity available in the market.
    pub liquidity: i64,
    /// Open interest in the market.
    pub open_interest: i64,
    /// Result of the market settlement.
    pub result: SettlementResult,
    /// Cap strike price, if applicable.
    pub cap_strike: Option<f64>,
    /// Indicator if the market can close early.
    pub can_close_early: bool,
    /// Value at expiration.
    pub expiration_value: String,
    /// Category of the market.
    pub category: String,
    /// Risk limit in cents.
    pub risk_limit_cents: i64,
    /// Type of strike, if applicable.
    pub strike_type: Option<String>,
    /// Floor strike price, if applicable.
    pub floor_strike: Option<f64>,
    /// Primary rules for the market.
    pub rules_primary: String,
    /// Secondary rules for the market.
    pub rules_secondary: String,
    /// Settlement value for the market.
    pub settlement_value: Option<String>,
    /// Functional strike information, if applicable.
    pub functional_strike: Option<String>,
}

/// An event in the Kalshi exchange.
///
/// This struct contains information about a specific event, including its identifier,
/// title, and other relevant details. It may also include associated markets.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    /// Unique identifier for the event.
    pub event_ticker: String,
    /// Ticker of the associated series.
    pub series_ticker: String,
    /// Subtitle of the event.
    pub sub_title: String,
    /// Title of the event.
    pub title: String,
    /// Indicates if the event's outcomes are mutually exclusive.
    pub mutually_exclusive: bool,
    /// Category of the event.
    pub category: String,
    /// Optional list of markets associated with this event.
    pub markets: Option<Vec<Market>>,
    /// Optional date of the event's occurrence.
    pub strike_date: Option<String>,
    /// Optional period of the event.
    pub strike_period: Option<String>,
}

/// Series on the Kalshi exchange.
///
/// This struct includes details about a specific series, such as its frequency,
/// title, and category. It also includes information on settlement sources and
/// related contract URLs.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Series {
    /// Unique ticker identifying the series.
    pub ticker: String,
    /// Frequency of the series.
    pub frequency: String,
    /// Title of the series.
    pub title: String,
    /// Category of the series.
    pub category: String,
    /// Tags associated with the series.
    pub tags: Vec<String>,
    /// Sources used for settling the series.
    pub settlement_sources: Vec<SettlementSource>,
    /// URL of the contract related to the series.
    pub contract_url: String,
}

/// A source of a settlement in the Kalshi exchange.
///
/// This struct contains information about a source used for settling a series, including the source's URL and name.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct SettlementSource {
    /// URL of the settlement source.
    pub url: String,
    /// Name of the settlement source.
    pub name: String,
}

/// The order book of a market in the Kalshi exchange.
///
/// This struct includes the bid and ask prices for both 'Yes' and 'No' options in a market, structured as nested vectors.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Orderbook {
    /// Nested vector of bids and asks for the 'Yes' option.
    /// Each inner vector typically contains price and quantity.
    pub yes: Option<Vec<Vec<i32>>>,
    /// Nested vector of bids and asks for the 'No' option.
    /// Each inner vector typically contains price and quantity.
    pub no: Option<Vec<Vec<i32>>>,
}

/// Snapshot of market data in the Kalshi exchange.
///
/// This struct provides a snapshot of the market at a specific time, including prices, bids, asks, volume, and open interest.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Snapshot {
    /// Last traded price for the 'Yes' option.
    pub yes_price: i32,
    /// Current highest bid price for the 'Yes' option.
    pub yes_bid: i32,
    /// Current lowest ask price for the 'Yes' option.
    pub yes_ask: i32,
    /// Current highest bid price for the 'No' option.
    pub no_bid: i32,
    /// Current lowest ask price for the 'No' option.
    pub no_ask: i32,
    /// Total trading volume at the snapshot time.
    pub volume: i32,
    /// Open interest at the snapshot time.
    pub open_interest: i32,
    /// Timestamp of the snapshot.
    pub ts: i64,
}

/// A trade in the Kalshi exchange.
///
/// This struct contains details of an individual trade, including the trade ID, side, ticker, and executed prices.
///
/// Used in methods for retrieving user fills and specific trade details.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    /// Unique identifier of the trade.
    pub trade_id: String,
    /// Side of the taker in the trade (e.g., 'buyer' or 'seller').
    pub taker_side: String,
    /// Ticker of the market in which the trade occurred.
    pub ticker: String,
    /// Number of contracts or shares traded.
    pub count: i32,
    /// Executed price for the 'Yes' option.
    pub yes_price: i32,
    /// Executed price for the 'No' option.
    pub no_price: i32,
    /// Time when the trade was created.
    pub created_time: String,
}

/// Possible outcomes of a market settlement on the Kalshi exchange.
///
/// This enum represents the different results that can be assigned to a market
/// upon its conclusion.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SettlementResult {
    /// The outcome of the market is affirmative.
    Yes,
    /// The outcome of the market is negative.
    No,
    /// The market is voided, usually due to specific conditions not being met.
    #[serde(rename = "")]
    Void,
    /// All options in the market are settled as 'No'.
    #[serde(rename = "all_no")]
    AllNo,
    /// All options in the market are settled as 'Yes'.
    #[serde(rename = "all_yes")]
    AllYes,
}

/// The different statuses a market can have on the Kalshi exchange.
///
/// This enum is used to represent the current operational state of a market.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarketStatus {
    /// The market is open for trading.
    Open,

    /// The market is closed and not currently available for trading.
    Closed,

    /// The market has been settled, and the outcome is determined.
    Settled,
}
