// imports
use reqwest;
use serde::{Deserialize, Serialize};

// Main Implementation, plan to abstract out in the future
#[derive(Debug)]
pub struct Kalshi<'a> {
    base_url: &'a str,
    logged_in: bool,
    curr_token: Option<String>,
    member_id: Option<String>,
    client: reqwest::Client,
}

// MACROS

macro_rules! add_param {
    ($params:ident, $param_name:expr, $param_value:expr) => {
        if let Some(param) = $param_value {
            $params.push(($param_name, param.to_string()));
        }
    };
}

// HELPER FUNCTIONS
fn bool_to_str(value: bool) -> &'static str {
    match value {
        true => "true",
        false => "false",
    }
}

// METHODS
// -----------------------------------------------
impl<'a> Kalshi<'a> {
    pub fn new() -> Kalshi<'a> {
        return Kalshi {
            base_url: "",
            logged_in: false,
            curr_token: None,
            member_id: None,
            client: reqwest::Client::new(),
        };
    }

    pub fn build_base_url(&mut self, trading_env: TradingEnvironment) -> () {
        match trading_env {
            TradingEnvironment::LiveMarketMode => {
                self.base_url = "https://trading-api.kalshi.com/trade-api/v2";
            }
            TradingEnvironment::DemoMode => {
                self.base_url = "https://demo-api.kalshi.co/trade-api/v2";
            }
        }
    }

    pub async fn login(&mut self, user: &str, password: &str) -> Result<(), reqwest::Error> {
        let login_url: &str = &format!("{}/login", self.base_url.to_string());

        let login_payload = LoginPayload {
            email: user.to_string(),
            password: password.to_string(),
        };

        let result: LoginResponse = self
            .client
            .post(login_url)
            .json(&login_payload)
            .send()
            .await?
            .json()
            .await?;

        self.curr_token = Some(format!("Bearer {}", result.token));
        self.member_id = Some(result.member_id);
        self.logged_in = true;

        return Ok(());
    }

    pub async fn logout(&self) -> Result<(), reqwest::Error> {
        let logout_url: &str = &format!("{}/logout", self.base_url.to_string());

        self.client
            .post(logout_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .header("content-type", "application/json".to_string())
            .send()
            .await?;

        return Ok(());
    }

    pub async fn get_balance(&self) -> Result<i64, reqwest::Error> {
        let balance_url: &str = &format!("{}/portfolio/balance", self.base_url.to_string());

        let result: BalanceResponse = self
            .client
            .get(balance_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .send()
            .await?
            .json()
            .await?;

        let bal = result.balance;

        return Ok(bal);
    }

    pub async fn get_exchange_status(&self) -> Result<ExchangeStatus, reqwest::Error> {
        let exchange_status_url: &str = &format!("{}/exchange/status", self.base_url.to_string());

        let result: ExchangeStatus = self
            .client
            .get(exchange_status_url)
            .send()
            .await?
            .json()
            .await?;

        return Ok(result);
    }

    pub async fn get_exchange_schedule(&self) -> Result<ExchangeScheduleStandard, reqwest::Error> {
        let exchange_schedule_url: &str =
            &format!("{}/exchange/schedule", self.base_url.to_string());

        let result: ExchangeScheduleResponse = self
            .client
            .get(exchange_schedule_url)
            .send()
            .await?
            .json()
            .await?;
        return Ok(result.schedule);
    }

    // WIP NOT FINISHED YET
    pub async fn get_multiple_fills(&self) -> Result<Vec<Fill>, reqwest::Error> {
        let user_fills_url: &str = &format!("{}/portfolio/fills", self.base_url.to_string());
        // TODO: NOT FULLY FEATURED YET

        let result: MultipleFillsResponse = self
            .client
            .get(user_fills_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .send()
            .await?
            .json()
            .await?;

        return Ok(result.fills);
    }

    // WIP NOT FINISHED YET
    pub async fn get_multiple_orders(&self) -> Result<Vec<Order>, reqwest::Error> {
        // TODO: NOT FULly FEATURED YET
        let user_orders_url: &str = &format!("{}/portfolio/orders", self.base_url.to_string());

        let result: MultipleOrderResponse = self
            .client
            .get(user_orders_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .send()
            .await?
            .json()
            .await?;

        return Ok(result.orders);
    }

    pub async fn get_single_order(&self, order_id: &String) -> Result<Order, reqwest::Error> {
        let user_order_url: &str = &format!(
            "{}/portfolio/orders/{}",
            self.base_url.to_string(),
            order_id
        );

        let result: SingleOrderResponse = self
            .client
            .get(user_order_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .send()
            .await?
            .json()
            .await?;

        return Ok(result.order);
    }

    pub async fn get_single_event(
        &self,
        event_ticker: &String,
        with_nested_markets: Option<bool>,
    ) -> Result<Event, reqwest::Error> {
        let single_event_url: &str =
            &format!("{}/events/{}", self.base_url.to_string(), event_ticker);

        let mut params: Vec<(&str, String)> = Vec::new();

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

        let mut params: Vec<(&str, String)> = Vec::new();

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

        let mut params: Vec<(&str, String)> = Vec::new();

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

        let mut params: Vec<(&str, String)> = Vec::new();

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

        println!("{:?}", result);

        Ok((result.cursor, result.markets))
    }

    pub fn get_user_token(&self) -> Option<String> {
        match &self.curr_token {
            Some(val) => return Some(val.clone()),
            _ => return None,
        }
    }
}

// STRUCTS
// -----------------------------------------------

// PRIVATE STRUCTS INTENDED FOR INTERNAL USE ONLY
// -----------------------------------------------

// used in login method
#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    member_id: String,
    token: String,
}
// used in login method
#[derive(Debug, Serialize, Deserialize)]
struct LoginPayload {
    email: String,
    password: String,
}
// used in getbalance method
#[derive(Debug, Serialize, Deserialize)]
struct BalanceResponse {
    balance: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct SingleOrderResponse {
    order: Order,
}

// used in get_user_fills
#[derive(Debug, Deserialize, Serialize)]
struct MultipleFillsResponse {
    fills: Vec<Fill>,
}

// used in get_user_orders
#[derive(Debug, Deserialize, Serialize)]
struct MultipleOrderResponse {
    orders: Vec<Order>,
}

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

// used in get_exchange_schedule
#[derive(Debug, Deserialize, Serialize)]
struct ExchangeScheduleResponse {
    schedule: ExchangeScheduleStandard,
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

#[derive(Debug, Deserialize, Serialize)]
struct PublicMarketsResponse {
    cursor: Option<String>,
    markets: Vec<Market>,
}

// PUBLIC STRUCTS AVAILABLE TO USER
// -----------------------------------------------

// used in get_exchange_status
#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeStatus {
    pub trading_active: bool,
    pub exchange_active: bool,
}

// used in get_exchange_schedule
#[derive(Debug, Deserialize, Serialize)]
pub struct DaySchedule {
    pub open_time: String,
    pub close_time: String,
}

// used in get_exchange_schedule
#[derive(Debug, Deserialize, Serialize)]
pub struct StandardHours {
    pub monday: DaySchedule,
    pub tuesday: DaySchedule,
    pub wednesday: DaySchedule,
    pub thursday: DaySchedule,
    pub friday: DaySchedule,
    pub saturday: DaySchedule,
    pub sunday: DaySchedule,
}

// used in get_exchange_schedule
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeScheduleStandard {
    pub standard_hours: StandardHours,
    pub maintenance_windows: Vec<String>,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Fill {
    pub action: String,
    pub count: i32,
    pub created_time: String,
    pub is_taker: bool,
    pub no_price: i64,
    pub order_id: String,
    pub side: String,
    pub ticker: String,
    pub trade_id: String,
    pub yes_price: i64,
}

// used in get_user_orders and get_order

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub order_id: String,
    pub user_id: String,
    pub ticker: String,
    pub status: String,
    pub yes_price: i32,
    pub no_price: i32,
    pub created_time: String,
    pub taker_fill_count: i32,
    pub taker_fill_cost: i32,
    pub place_count: i32,
    pub decrease_count: i32,
    pub maker_fill_count: i32,
    pub fcc_cancel_count: i32,
    pub close_cancel_count: i32,
    pub remaining_count: i32,
    pub queue_position: i32,
    pub expiration_time: String,
    pub taker_fees: i32,
    pub action: String,
    pub side: String,
    pub r#type: String,
    pub last_update_time: String,
    pub client_order_id: String,
    pub order_group_id: String,
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
}

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
    pub expiration_time: String,
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
    pub result: String,
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

// used in get_series
#[derive(Debug, Deserialize, Serialize)]
pub struct Series {
    pub ticker: String,
    pub frequency: String,
    pub title: String,
    pub category: String,
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

// ENUMS (Custom Errors Planned)
// -----------------------------------------------
pub enum TradingEnvironment {
    DemoMode,
    LiveMarketMode,
}

// unit tests, absent at the moment. all test logic is handled in the test bot dir
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
