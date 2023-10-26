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

    pub async fn get_exchange_schedule(&self) -> Result<ExchangeSchedule, reqwest::Error> {
        let exchange_schedule_url: &str =
            &format!("{}/exchange/schedule", self.base_url.to_string());

        let result: ExchangeSchedule = self
            .client
            .get(exchange_schedule_url)
            .send()
            .await?
            .json()
            .await?;
        return Ok(result);
    }

    // WIP NOT FINISHED YET
    pub async fn get_multiple_fills(&self) -> Result<Vec<Trade>, reqwest::Error> {
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
        todo!()
    }


    pub async fn get_single_market(&self, ticker: &str) -> Result<Market, reqwest::Error> {
        
        todo!()
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
    fills: Vec<Trade>,
}

// used in get_user_orders
#[derive(Debug, Deserialize, Serialize)]
struct MultipleOrderResponse {
    orders: Vec<Order>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SingleEventResponse {
    event: Event,
}

// PUBLIC STRUCTS AVAILABLE TO USER
// -----------------------------------------------

// used in get_exchange_status
#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeStatus {
    trading_active: bool,
    exchange_active: bool,
}

// used in get_exchange_schedule
#[derive(Debug, Deserialize, Serialize)]
pub struct DaySchedule {
    open_time: String,
    close_time: String,
}

// used in get_exchange_schedule
#[derive(Debug, Deserialize, Serialize)]
pub struct StandardHours {
    monday: DaySchedule,
    tuesday: DaySchedule,
    wednesday: DaySchedule,
    thursday: DaySchedule,
    friday: DaySchedule,
    saturday: DaySchedule,
    sunday: DaySchedule,
}

// used in get_exchange_schedule
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeScheduleStandard {
    standard_hours: StandardHours,
    maintenance_windows: Vec<String>,
}

// used in get_exchange_schedule
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeSchedule {
    schedule: ExchangeScheduleStandard,
}

// used in get_user_fills and get_fill
#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    trade_id: String,
    ticker: String,
    order_id: String,
    side: String,
    action: String,
    count: i32,
    yes_price: i32,
    no_price: i32,
    is_taker: bool,
    created_time: String,
}

// used in get_user_orders and get_order

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub order_id: String,
    user_id: String,
    ticker: String,
    status: String,
    yes_price: i32,
    no_price: i32,
    created_time: String,
    taker_fill_count: i32,
    taker_fill_cost: i32,
    place_count: i32,
    decrease_count: i32,
    maker_fill_count: i32,
    fcc_cancel_count: i32,
    close_cancel_count: i32,
    remaining_count: i32,
    queue_position: i32,
    expiration_time: String,
    taker_fees: i32,
    action: String,
    side: String,
    r#type: String,
    last_update_time: String,
    client_order_id: String,
    order_group_id: String,
}

// used in get_event and get_events methods
#[derive(Debug, Deserialize, Serialize)]

pub struct Event {
    event_ticker: String,
    series_ticker: String,
    sub_title: String,
    title: String,
    mutually_exclusive: bool,
    category: String,
    markets: Option<Vec<Market>>,
}

// used in get_market and get_markets and get_events method
#[derive(Debug, Deserialize, Serialize)]
pub struct Market {
    ticker: String,
    event_ticker: String,
    market_type: String,
    title: String,
    subtitle: String,
    yes_sub_title: String,
    no_sub_title: String,
    open_time: String,
    close_time: String,
    expiration_time: String,
    latest_expiration_time: String,
    settlement_timer_seconds: u32,
    status: String,
    response_price_units: String,
    notional_value: u32,
    tick_size: u32,
    yes_bid: u32,
    yes_ask: u32,
    no_bid: u32,
    no_ask: u32,
    last_price: u32,
    previous_yes_bid: u32,
    previous_yes_ask: u32,
    previous_price: u32,
    volume: u32,
    volume_24h: u32,
    liquidity: u32,
    open_interest: u32,
    result: String,
    settlement_value: u32,
    can_close_early: bool,
    expiration_value: String,
    category: String,
    risk_limit_cents: u32,
    rules_primary: String,
    rules_secondary: String,
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
