use super::Kalshi;
use crate::kalshi_error::*;
use std::fmt;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

impl<'a> Kalshi<'a> {
    pub async fn get_balance(&self) -> Result<i64, KalshiError> {
        if self.curr_token == None {
            return Err(KalshiError::UserInputError(
                "Not logged in, a valid token is required for requests that require authentication"
                    .to_string(),
            ));
        }

        let balance_url: &str = &format!("{}/portfolio/balance", self.base_url.to_string());

        let result: BalanceResponse = self
            .client
            .get(balance_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .send()
            .await?
            .json()
            .await?;

        Ok(result.balance)
    }

    pub async fn get_multiple_orders(
        &self,
        ticker: Option<String>,
        event_ticker: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
        status: Option<String>,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<Order>), KalshiError> {
        if self.curr_token == None {
            return Err(KalshiError::UserInputError(
                "Not logged in, a valid token is required for requests that require authentication"
                    .to_string(),
            ));
        }
        let user_orders_url: &str = &format!("{}/portfolio/orders", self.base_url.to_string());

        let mut params: Vec<(&str, String)> = Vec::with_capacity(7);

        add_param!(params, "ticker", ticker);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "min_ts", min_ts);
        add_param!(params, "max_ts", max_ts);
        add_param!(params, "event_ticker", event_ticker);
        add_param!(params, "status", status);

        let user_orders_url = reqwest::Url::parse_with_params(user_orders_url, &params)
            .unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                panic!("Internal Parse Error, please contact developer!");
            });

        let result: MultipleOrderResponse = self
            .client
            .get(user_orders_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .send()
            .await?
            .json()
            .await?;

        return Ok((result.cursor, result.orders));
    }

    pub async fn get_single_order(&self, order_id: &String) -> Result<Order, KalshiError> {
        if self.curr_token == None {
            return Err(KalshiError::UserInputError(
                "Not logged in, a valid token is required for requests that require authentication"
                    .to_string(),
            ));
        }
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

    pub async fn cancel_order(&self, order_id: &str) -> Result<(Order, i32), KalshiError> {
        if self.curr_token == None {
            return Err(KalshiError::UserInputError(
                "Not logged in, a valid token is required for requests that require authentication"
                    .to_string(),
            ));
        }
        let cancel_order_url: &str = &format!(
            "{}/portfolio/orders/{}",
            self.base_url.to_string(),
            order_id
        );

        let result: DeleteOrderResponse = self
            .client
            .delete(cancel_order_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .send()
            .await?
            .json()
            .await?;

        Ok((result.order, result.reduced_by))
    }

    pub async fn decrease_order(
        &self,
        order_id: &str,
        reduce_by: Option<i32>,
        reduce_to: Option<i32>,
    ) -> Result<Order, KalshiError> {
        if self.curr_token == None {
            return Err(KalshiError::UserInputError(
                "Not logged in, a valid token is required for requests that require authentication"
                    .to_string(),
            ));
        }
        let decrease_order_url: &str = &format!(
            "{}/portfolio/orders/{}",
            self.base_url.to_string(),
            order_id
        );

        match (reduce_by, reduce_to) {
            (Some(_), Some(_)) => {
                return Err(KalshiError::UserInputError(
                    "Can only provide reduce_by strict exclusive or reduce_to, can't provide both"
                        .to_string(),
                ));
            }
            (None, None) => {
                return Err(KalshiError::UserInputError(
                    "Must provide either reduce_by exclusive or reduce_to, can't provide neither"
                        .to_string(),
                ));
            }
            _ => {}
        }

        let decrease_payload = DecreaseOrderPayload {
            reduce_by: reduce_by,
            reduce_to: reduce_to,
        };

        let result: SingleOrderResponse = self
            .client
            .post(decrease_order_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .header("content-type", "application/json".to_string())
            .json(&decrease_payload)
            .send()
            .await?
            .json()
            .await?;

        Ok(result.order)
    }

    pub async fn get_multiple_fills(
        &self,
        ticker: Option<String>,
        order_id: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<Fill>), KalshiError> {
        if self.curr_token == None {
            return Err(KalshiError::UserInputError(
                "Not logged in, a valid token is required for requests that require authentication"
                    .to_string(),
            ));
        }
        let user_fills_url: &str = &format!("{}/portfolio/fills", self.base_url.to_string());

        let mut params: Vec<(&str, String)> = Vec::with_capacity(7);

        add_param!(params, "ticker", ticker);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "min_ts", min_ts);
        add_param!(params, "max_ts", max_ts);
        add_param!(params, "order_id", order_id);

        let user_fills_url = reqwest::Url::parse_with_params(user_fills_url, &params)
            .unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                panic!("Internal Parse Error, please contact developer!");
            });

        let result: MultipleFillsResponse = self
            .client
            .get(user_fills_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .send()
            .await?
            .json()
            .await?;

        return Ok((result.cursor, result.fills));
    }

    pub async fn get_portfolio_settlements(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<Settlement>), KalshiError> {
        if self.curr_token == None {
            return Err(KalshiError::UserInputError(
                "Not logged in, a valid token is required for requests that require authentication"
                    .to_string(),
            ));
        }
        let settlements_url: &str = &format!("{}/portfolio/settlements", self.base_url.to_string());

        let mut params: Vec<(&str, String)> = Vec::with_capacity(6);

        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);

        let settlements_url = reqwest::Url::parse_with_params(settlements_url, &params)
            .unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                panic!("Internal Parse Error, please contact developer!");
            });

        let result: PortfolioSettlementResponse = self
            .client
            .get(settlements_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .send()
            .await?
            .json()
            .await?;

        Ok((result.cursor, result.settlements))
    }

    pub async fn get_user_positions(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        settlement_status: Option<String>,
        ticker: Option<String>,
        event_ticker: Option<String>,
    ) -> Result<(Option<String>, Vec<EventPosition>, Vec<MarketPosition>), KalshiError> {
        if self.curr_token == None {
            return Err(KalshiError::UserInputError(
                "Not logged in, a valid token is required for requests that require authentication"
                    .to_string(),
            ));
        }
        let positions_url: &str = &format!("{}/portfolio/positions", self.base_url.to_string());

        let mut params: Vec<(&str, String)> = Vec::with_capacity(6);

        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "settlement_status", settlement_status);
        add_param!(params, "ticker", ticker);
        add_param!(params, "event_ticker", event_ticker);

        let positions_url =
            reqwest::Url::parse_with_params(positions_url, &params).unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                panic!("Internal Parse Error, please contact developer!");
            });

        let result: GetPositionsResponse = self
            .client
            .get(positions_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .send()
            .await?
            .json()
            .await?;

        Ok((
            result.cursor,
            result.event_positions,
            result.market_positions,
        ))
    }

    pub async fn create_order(
        &self,
        action: Action,
        client_order_id: Option<String>,
        count: i32,
        side: Side,
        ticker: String,
        input_type: OrderType,
        buy_max_cost: Option<i64>,
        expiration_ts: Option<i64>,
        no_price: Option<i64>,
        sell_position_floor: Option<i32>,
        yes_price: Option<i64>,
    ) -> Result<Order, KalshiError> {
        if self.curr_token == None {
            return Err(KalshiError::UserInputError(
                "Not logged in, a valid token is required for requests that require authentication"
                    .to_string(),
            ));
        }
        let order_url: &str = &format!("{}/portfolio/orders", self.base_url.to_string());

        match input_type {
            OrderType::Limit => match (no_price, yes_price) {
                (Some(_), Some(_)) => {
                    return Err(KalshiError::UserInputError(
                        "Can only provide no_price exclusive or yes_price, can't provide both"
                            .to_string(),
                    ));
                }
                (None, None) => {
                    return Err(KalshiError::UserInputError(
                            "Must provide either no_price exclusive or yes_price, can't provide neither"
                                .to_string(),
                        ));
                }
                _ => {}
            },
            _ => {}
        }

        let unwrapped_id = match client_order_id {
            Some(id) => id,
            _ => String::from(Uuid::new_v4()),
        };

        let order_payload = CreateOrderPayload {
            action: action,
            client_order_id: unwrapped_id,
            count: count,
            side: side,
            ticker: ticker,
            r#type: input_type,
            buy_max_cost: buy_max_cost,
            expiration_ts: expiration_ts,
            no_price: no_price,
            sell_position_floor: sell_position_floor,
            yes_price: yes_price,
        };

        let response: SingleOrderResponse = self
            .client
            .post(order_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .header("content-type", "application/json".to_string())
            .json(&order_payload)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.order)
    }
}

// PRIVATE STRUCTS
// used in getbalance method
#[derive(Debug, Serialize, Deserialize)]
struct BalanceResponse {
    balance: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct SingleOrderResponse {
    order: Order,
}

#[derive(Debug, Deserialize, Serialize)]
struct MultipleOrderResponse {
    orders: Vec<Order>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DeleteOrderResponse {
    order: Order,
    reduced_by: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct DecreaseOrderResponse {
    order: Order,
}

#[derive(Debug, Deserialize, Serialize)]
struct DecreaseOrderPayload {
    reduce_by: Option<i32>,
    reduce_to: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MultipleFillsResponse {
    fills: Vec<Fill>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PortfolioSettlementResponse {
    cursor: Option<String>,
    settlements: Vec<Settlement>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetPositionsResponse {
    cursor: Option<String>,
    event_positions: Vec<EventPosition>,
    market_positions: Vec<MarketPosition>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateOrderPayload {
    action: Action,
    client_order_id: String,
    count: i32,
    side: Side,
    ticker: String,
    r#type: OrderType,
    buy_max_cost: Option<i64>,
    expiration_ts: Option<i64>,
    no_price: Option<i64>,
    sell_position_floor: Option<i32>,
    yes_price: Option<i64>,
}

// PUBLIC STRUCTS
#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub order_id: String,
    pub user_id: Option<String>,
    pub ticker: String,
    pub status: OrderStatus,
    pub yes_price: i32,
    pub no_price: i32,
    pub created_time: Option<String>,
    pub taker_fill_count: Option<i32>,
    pub taker_fill_cost: Option<i32>,
    pub place_count: Option<i32>,
    pub decrease_count: Option<i32>,
    pub maker_fill_count: Option<i32>,
    pub fcc_cancel_count: Option<i32>,
    pub close_cancel_count: Option<i32>,
    pub remaining_count: Option<i32>,
    pub queue_position: Option<i32>,
    pub expiration_time: Option<String>,
    pub taker_fees: Option<i32>,
    pub action: Action,
    pub side: Side,
    pub r#type: String,
    pub last_update_time: Option<String>,
    pub client_order_id: String,
    pub order_group_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Fill {
    pub action: Action,
    pub count: i32,
    pub created_time: String,
    pub is_taker: bool,
    pub no_price: i64,
    pub order_id: String,
    pub side: Side,
    pub ticker: String,
    pub trade_id: String,
    pub yes_price: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settlement {
    pub market_result: String,
    pub no_count: i64,
    pub no_total_cost: i64,
    pub revenue: i64,
    pub settled_time: String,
    pub ticker: String,
    pub yes_count: i64,
    pub yes_total_cost: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventPosition {
    pub event_exposure: i64,
    pub event_ticker: String,
    pub fees_paid: i64,
    pub realized_pnl: i64,
    pub resting_order_count: i32,
    pub total_cost: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketPosition {
    pub fees_paid: i64,
    pub market_exposure: i64,
    pub position: i32,
    pub realized_pnl: i64,
    pub resting_orders_count: i32,
    pub ticker: String,
    pub total_traded: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Yes,
    No,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Buy,
    Sell,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Buy => write!(f, "buy"),
            Action::Sell => write!(f, "sell"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Resting,
    Canceled,
    Executed,
    Pending,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::Resting => write!(f, "resting"),
            OrderStatus::Canceled => write!(f, "cancelled"),
            OrderStatus::Executed => write!(f, "executed"),
            OrderStatus::Pending => write!(f, "pending"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Market,
    Limit,
}
