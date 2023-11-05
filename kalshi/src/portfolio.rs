use super::Kalshi;
use crate::kalshi_error::*;

use serde::{Deserialize, Serialize};

impl<'a> Kalshi<'a> {
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

        return Ok(result.balance);
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
    ) -> Result<(Option<String>, Vec<Order>), reqwest::Error> {
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



    pub async fn cancel_order(&self, order_id: &str) -> Result<(Order, i32), reqwest::Error> {
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
        let decrease_order_url: &str = &format!(
            "{}/portfolio/orders/{}",
            self.base_url.to_string(),
            order_id
        );

        match (reduce_by, reduce_to) {
            (Some(_), Some(_)) => {
                return Err(KalshiError::UserInputError(
                    "Can only provide reduce_by strict exclusive or reduce_to, can't provide both".to_string(),
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
    ) -> Result<(Option<String>, Vec<Fill>), reqwest::Error> {
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

// PUBLIC STRUCTS
#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub order_id: String,
    pub user_id: Option<String>,
    pub ticker: String,
    pub status: String,
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
    pub action: String,
    pub side: String,
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
