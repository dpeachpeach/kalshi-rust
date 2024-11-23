use super::Kalshi;
use crate::{kalshi_error::*, LoggedIn};
use std::fmt;
use std::sync::Arc;
use tokio::task;
use uuid::Uuid;

use serde::{Deserialize, Deserializer, Serialize};

impl<'a> Kalshi<LoggedIn> {
    /// Retrieves the current balance of the authenticated user from the Kalshi exchange.
    ///
    /// This method fetches the user's balance, requiring a valid authentication token.
    /// If the user is not logged in or the token is missing, it returns an error.
    ///
    /// # Returns
    ///
    /// - `Ok(i64)`: The user's current balance on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let balance = kalshi_instance.get_balance().await.unwrap();
    /// ```
    ///
    pub async fn get_balance(&self) -> Result<i64, KalshiError> {
        let balance_url: &str = &format!("{}/portfolio/balance", self.base_url);

        let result: BalanceResponse = self
            .client
            .get(balance_url)
            .header("Authorization", self.state.curr_token.clone())
            .send()
            .await?
            .json()
            .await?;

        Ok(result.balance)
    }

    /// Retrieves a list of orders from the Kalshi exchange based on specified criteria.
    ///
    /// This method fetches multiple orders, allowing for filtering by ticker, event ticker, time range,
    /// status, and pagination. A valid authentication token is required to access this information.
    /// If the user is not logged in or the token is missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `ticker` - An optional string to filter orders by market ticker.
    /// * `event_ticker` - An optional string to filter orders by event ticker.
    /// * `min_ts` - An optional minimum timestamp for order creation time.
    /// * `max_ts` - An optional maximum timestamp for order creation time.
    /// * `status` - An optional string to filter orders by their status.
    /// * `limit` - An optional integer to limit the number of orders returned.
    /// * `cursor` - An optional string for pagination cursor.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Order>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Order` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    /// Retrieves all possible orders (Will crash, need to limit for a successful request).
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let orders = kalshi_instance.get_multiple_orders(
    ///     Some("ticker_name"), None, None, None, None, None, None
    /// ).await.unwrap();
    /// ```
    ///
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
        let user_orders_url: &str = &format!("{}/portfolio/orders", self.base_url);

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
            .header("Authorization", self.state.curr_token.clone())
            .send()
            .await?
            .json()
            .await?;

        Ok((result.cursor, result.orders))
    }

    /// Retrieves detailed information about a specific order from the Kalshi exchange.
    ///
    /// This method fetches data for a single order identified by its order ID. A valid authentication token
    /// is required to access this information. If the user is not logged in or the token is missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `order_id` - A reference to a string representing the order's unique identifier.
    ///
    /// # Returns
    ///
    /// - `Ok(Order)`: Detailed information about the specified order on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let order_id = "some_order_id";
    /// let order = kalshi_instance.get_single_order(&order_id).await.unwrap();
    /// ```
    ///
    pub async fn get_single_order(&self, order_id: &String) -> Result<Order, KalshiError> {
        let user_order_url: &str = &format!("{}/portfolio/orders/{}", self.base_url, order_id);

        let result: SingleOrderResponse = self
            .client
            .get(user_order_url)
            .header("Authorization", self.state.curr_token.clone())
            .send()
            .await?
            .json()
            .await?;

        Ok(result.order)
    }

    /// Cancels an existing order on the Kalshi exchange.
    ///
    /// This method cancels an order specified by its ID. A valid authentication token is
    /// required to perform this action. If the user is not logged in or the token is missing,
    /// it returns an error.
    ///
    /// # Arguments
    ///
    /// * `order_id` - A string slice referencing the ID of the order to be canceled.
    ///
    /// # Returns
    ///
    /// - `Ok((Order, i32))`: A tuple containing the updated `Order` object after cancellation
    ///   and an integer indicating the amount by which the order was reduced on successful cancellation.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let order_id = "some_order_id";
    /// let (order, reduced_by) = kalshi_instance.cancel_order(order_id).await.unwrap();
    /// ```
    ///
    pub async fn cancel_order(&self, order_id: &str) -> Result<(Order, i32), KalshiError> {
        let cancel_order_url: &str = &format!("{}/portfolio/orders/{}", self.base_url, order_id);

        let result: DeleteOrderResponse = self
            .client
            .delete(cancel_order_url)
            .header("Authorization", self.state.curr_token.clone())
            .send()
            .await?
            .json()
            .await?;

        Ok((result.order, result.reduced_by))
    }
    /// Decreases the size of an existing order on the Kalshi exchange.
    ///
    /// This method allows reducing the size of an order either by specifying the amount to reduce
    /// (`reduce_by`) or setting a new target size (`reduce_to`). A valid authentication token is
    /// required for this operation. It's important to provide either `reduce_by` or `reduce_to`,
    /// but not both at the same time.
    ///
    /// # Arguments
    ///
    /// * `order_id` - A string slice referencing the ID of the order to be decreased.
    /// * `reduce_by` - An optional integer specifying how much to reduce the order by.
    /// * `reduce_to` - An optional integer specifying the new size of the order.
    ///
    /// # Returns
    ///
    /// - `Ok(Order)`: The updated `Order` object after decreasing the size.
    /// - `Err(KalshiError)`: An error if the user is not authenticated, if both `reduce_by` and `reduce_to` are provided,
    ///   or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let order_id = "some_order_id";
    /// let decrease_result = kalshi_instance.decrease_order(order_id, Some(5), None).await.unwrap();
    /// ```
    ///
    pub async fn decrease_order(
        &self,
        order_id: &str,
        reduce_by: Option<i32>,
        reduce_to: Option<i32>,
    ) -> Result<Order, KalshiError> {
        let decrease_order_url: &str = &format!("{}/portfolio/orders/{}", self.base_url, order_id);

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
            reduce_by,
            reduce_to,
        };

        let result: SingleOrderResponse = self
            .client
            .post(decrease_order_url)
            .header("Authorization", self.state.curr_token.clone())
            .header("content-type", "application/json".to_string())
            .json(&decrease_payload)
            .send()
            .await?
            .json()
            .await?;

        Ok(result.order)
    }

    /// Retrieves a list of fills from the Kalshi exchange based on specified criteria.
    ///
    /// This method fetches multiple fills, allowing for filtering by ticker, order ID, time range,
    /// and pagination. A valid authentication token is required to access this information.
    /// If the user is not logged in or the token is missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `ticker` - An optional string to filter fills by market ticker.
    /// * `order_id` - An optional string to filter fills by order ID.
    /// * `min_ts` - An optional minimum timestamp for fill creation time.
    /// * `max_ts` - An optional maximum timestamp for fill creation time.
    /// * `limit` - An optional integer to limit the number of fills returned.
    /// * `cursor` - An optional string for pagination cursor.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Fill>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Fill` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    /// Retrieves all filled orders
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let fills = kalshi_instance.get_multiple_fills(
    ///     Some("ticker_name"), None, None, None, None, None
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_multiple_fills(
        &self,
        ticker: Option<String>,
        order_id: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<Fill>), KalshiError> {
        let user_fills_url: &str = &format!("{}/portfolio/fills", self.base_url);

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
            .header("Authorization", self.state.curr_token.clone())
            .send()
            .await?
            .json()
            .await?;

        Ok((result.cursor, result.fills))
    }

    /// Retrieves a list of portfolio settlements from the Kalshi exchange.
    ///
    /// This method fetches settlements in the user's portfolio, with options for pagination using limit and cursor.
    /// A valid authentication token is required to access this information.
    /// If the user is not logged in or the token is missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of settlements returned.
    /// * `cursor` - An optional string for pagination cursor.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Settlement>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Settlement` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let settlements = kalshi_instance.get_portfolio_settlements(None, None).await.unwrap();
    /// ```
    ///
    pub async fn get_portfolio_settlements(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<Settlement>), KalshiError> {
        let settlements_url: &str = &format!("{}/portfolio/settlements", self.base_url);

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
            .header("Authorization", self.state.curr_token.clone())
            .send()
            .await?
            .json()
            .await?;

        Ok((result.cursor, result.settlements))
    }

    /// Retrieves the user's positions in events and markets from the Kalshi exchange.
    ///
    /// This method fetches the user's positions, providing options for filtering by settlement status,
    /// specific ticker, and event ticker, as well as pagination using limit and cursor. A valid
    /// authentication token is required to access this information. If the user is not logged in
    /// or the token is missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of positions returned.
    /// * `cursor` - An optional string for pagination cursor.
    /// * `settlement_status` - An optional string to filter positions by their settlement status.
    /// * `ticker` - An optional string to filter positions by market ticker.
    /// * `event_ticker` - An optional string to filter positions by event ticker.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<EventPosition>, Vec<MarketPosition>))`: A tuple containing an optional pagination cursor,
    ///   a vector of `EventPosition` objects, and a vector of `MarketPosition` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let user_positions = kalshi_instance.get_user_positions(None, None, None, None, None).await.unwrap();
    /// ```
    ///
    pub async fn get_user_positions(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        settlement_status: Option<String>,
        ticker: Option<String>,
        event_ticker: Option<String>,
    ) -> Result<(Option<String>, Vec<EventPosition>, Vec<MarketPosition>), KalshiError> {
        let positions_url: &str = &format!("{}/portfolio/positions", self.base_url);

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
            .header("Authorization", self.state.curr_token.clone())
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

    /// Submits an order to the Kalshi exchange.
    ///
    /// This method allows placing an order in the market, requiring details such as action, count, side,
    /// ticker, order type, and other optional parameters. A valid authentication token is
    /// required for this operation. Note that for limit orders, either `no_price` or `yes_price` must be provided,
    /// but not both.
    ///
    /// # Arguments
    ///
    /// * `action` - The action (buy/sell) of the order.
    /// * `client_order_id` - An optional client-side identifier for the order.
    /// * `count` - The number of shares or contracts to trade.
    /// * `side` - The side (Yes/No) of the order.
    /// * `ticker` - The market ticker the order is placed in.
    /// * `input_type` - The type of the order (e.g., market, limit).
    /// * `buy_max_cost` - The maximum cost for a buy order. Optional.
    /// * `expiration_ts` - The expiration timestamp for the order. Optional.
    /// * `no_price` - The price for the 'No' option in a limit order. Optional.
    /// * `sell_position_floor` - The minimum position size to maintain after selling. Optional.
    /// * `yes_price` - The price for the 'Yes' option in a limit order. Optional.
    ///
    /// # Returns
    ///
    /// - `Ok(Order)`: The created `Order` object on successful placement.
    /// - `Err(KalshiError)`: An error if the user is not authenticated, if both `no_price` and `yes_price` are provided for limit orders,
    ///   or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let action = Action::Buy;
    /// let side = Side::Yes;
    /// let order = kalshi_instance.create_order(
    ///     action,
    ///     None,
    ///     10,
    ///     side,
    ///     "example_ticker",
    ///     OrderType::Limit,
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     Some(100)
    /// ).await.unwrap();
    /// ```
    ///

    // todo: rewrite using generics
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
        let order_url: &str = &format!("{}/portfolio/orders", self.base_url);

        if let OrderType::Limit = input_type {
            match (no_price, yes_price) {
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
            }
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

        let response = self
            .client
            .post(order_url)
            .header("Authorization", self.state.curr_token.clone())
            .header("content-type", "application/json".to_string())
            .json(&order_payload)
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<SingleOrderResponse>().await {
                        Ok(order_response) => Ok(order_response.order),
                        Err(json_err) => {
                            // Handle JSON decoding error
                            let error_message =
                                format!("Failed to decode JSON response: {}", json_err);
                            eprintln!("{}", error_message);
                            Err(KalshiError::InternalError(error_message))
                        }
                    }
                } else {
                    // Handle non-success HTTP status codes
                    let error_message = format!("HTTP Error: {}", resp.status());
                    eprintln!("{}", error_message);
                    Err(KalshiError::InternalError(error_message))
                }
            }
            Err(request_err) => {
                // Handle errors in sending the request
                let error_message = format!("Failed to send request: {}", request_err);
                eprintln!("{}", error_message);
                Err(KalshiError::InternalError(error_message))
            }
        }
    }

    pub async fn batch_cancel_order(
        &mut self,
        batch: Vec<String>,
    ) -> Result<Vec<Result<(Order, i32), KalshiError>>, KalshiError> {
        let temp_instance = Arc::new(self.clone());
        let mut futures = Vec::new();

        for order_id in batch {
            let kalshi_ref = Arc::clone(&temp_instance);
            let order_id = order_id.clone();

            let future = task::spawn(async move { kalshi_ref.cancel_order(&order_id).await });
            futures.push(future);
        }

        let mut outputs = Vec::new();

        // TODO: improve error process for joining, I don't believe it's specific enough.
        for future in futures {
            match future.await {
                Ok(result) => outputs.push(result),
                Err(e) => {
                    return Err(KalshiError::UserInputError(format!(
                        "Join of concurrent requests failed, check input or message developer: {}",
                        e
                    )));
                }
            }
        }
        Ok(outputs)
    }

    pub async fn batch_create_order(
        &mut self,
        _batch: Vec<OrderCreationField>,
    ) -> Result<Vec<Result<(Order, i32), KalshiError>>, KalshiError> {
        todo!()
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
    #[serde(deserialize_with = "empty_string_is_none")]
    cursor: Option<String>,
}

fn empty_string_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
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
// -------------------------

/// Represents an order in the Kalshi exchange.
///
/// This struct details an individual order, including its identification, status, prices, and various metrics related to its lifecycle.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    /// Unique identifier for the order.
    pub order_id: String,
    /// Identifier of the user who placed the order. Optional.
    pub user_id: Option<String>,
    /// Ticker of the market associated with the order.
    pub ticker: String,
    /// Current status of the order (e.g., resting, executed).
    pub status: OrderStatus,
    /// Price of the 'Yes' option in the order.
    pub yes_price: i32,
    /// Price of the 'No' option in the order.
    pub no_price: i32,
    /// Timestamp when the order was created. Optional.
    pub created_time: Option<String>,
    /// Count of fills where the order acted as a taker. Optional.
    pub taker_fill_count: Option<i32>,
    /// Total cost of taker fills. Optional.
    pub taker_fill_cost: Option<i32>,
    /// Count of order placements. Optional.
    pub place_count: Option<i32>,
    /// Count of order decreases. Optional.
    pub decrease_count: Option<i32>,
    /// Count of fills where the order acted as a maker. Optional.
    pub maker_fill_count: Option<i32>,
    /// Count of FCC (Financial Crime Compliance) cancellations. Optional.
    pub fcc_cancel_count: Option<i32>,
    /// Count of cancellations at market close. Optional.
    pub close_cancel_count: Option<i32>,
    /// Remaining count of the order. Optional.
    pub remaining_count: Option<i32>,
    /// Position of the order in the queue. Optional.
    pub queue_position: Option<i32>,
    /// Expiration time of the order. Optional.
    pub expiration_time: Option<String>,
    /// Fees incurred as a taker. Optional.
    pub taker_fees: Option<i32>,
    /// The action (buy/sell) of the order.
    pub action: Action,
    /// The side (Yes/No) of the order.
    pub side: Side,
    /// Type of the order (e.g., market, limit).
    pub r#type: String,
    /// Last update time of the order. Optional.
    pub last_update_time: Option<String>,
    /// Client-side identifier for the order.
    pub client_order_id: String,
    /// Group identifier for the order.
    pub order_group_id: String,
}

/// A completed transaction (a 'fill') in the Kalshi exchange.
///
/// This struct details a single fill instance, including the action taken, the quantity,
/// the involved prices, and the identifiers of the order and trade.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Fill {
    /// The action (buy/sell) of the fill.
    pub action: Action,
    /// The number of contracts or shares involved in the fill.
    pub count: i32,
    /// The timestamp when the fill was created.
    pub created_time: String,
    /// Indicates if the fill was made by a taker.
    pub is_taker: bool,
    /// The price of the 'No' option in the fill.
    pub no_price: i64,
    /// The identifier of the associated order.
    pub order_id: String,
    /// The side (Yes/No) of the fill.
    pub side: Side,
    /// The ticker of the market in which the fill occurred.
    pub ticker: String,
    /// The unique identifier of the trade.
    pub trade_id: String,
    /// The price of the 'Yes' option in the fill.
    pub yes_price: i64,
}

/// A settlement of a market position in the Kalshi exchange.
///
/// This struct provides details of a market settlement, including the result, quantities,
/// costs involved, and the timestamp of settlement.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Settlement {
    /// The result of the market settlement.
    pub market_result: String,
    /// The quantity involved in the 'No' position.
    pub no_count: i64,
    /// The total cost associated with the 'No' position.
    pub no_total_cost: i64,
    /// The revenue generated from the settlement, in cents.
    pub revenue: i64,
    /// The timestamp when the settlement occurred.
    pub settled_time: String,
    /// The ticker of the market that was settled.
    pub ticker: String,
    /// The quantity involved in the 'Yes' position.
    pub yes_count: i64,
    /// The total cost associated with the 'Yes' position, in cents.
    pub yes_total_cost: i64,
}

/// A user's position in a specific event on the Kalshi exchange.
///
/// Details the user's exposure, costs, profits, and the number of resting orders related to a particular event.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct EventPosition {
    /// The total exposure amount in the event.
    pub event_exposure: i64,
    /// The ticker of the event.
    pub event_ticker: String,
    /// The total fees paid in the event in cents.
    pub fees_paid: i64,
    /// The realized profit or loss in the event in cents.
    pub realized_pnl: i64,
    /// The count of resting (active but unfilled) orders in the event.
    pub resting_order_count: i32,
    /// The total cost incurred in the event in cents.
    pub total_cost: i64,
}

/// A user's position in a specific market on the Kalshi exchange.
///
/// This struct includes details about the user's market position, including exposure, fees,
/// profits, and the number of resting orders.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketPosition {
    /// The total fees paid in the market in cents.
    pub fees_paid: i64,
    /// The total exposure amount in the market.
    pub market_exposure: i64,
    /// The current position of the user in the market.
    pub position: i32,
    /// The realized profit or loss in the market in cents.
    pub realized_pnl: i64,
    /// The count of resting orders in the market.
    pub resting_orders_count: i32,
    /// The ticker of the market.
    pub ticker: String,
    /// The total traded amount in the market.
    pub total_traded: i64,
}

/// Represents the necessary fields for creating an order in the Kalshi exchange.
///
/// This struct is used to encapsulate all the data needed to create a new order. It includes details about the order type,
/// the action being taken (buy/sell), the market ticker, and various other optional parameters that can be specified
/// to fine-tune the order according to the user's needs.
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderCreationField {
    /// The action (buy/sell) of the order.
    pub action: Action,
    /// Client-side identifier for the order. Optional.
    pub client_order_id: Option<String>,
    /// The number of contracts or shares involved in the order.
    pub count: i32,
    /// The side (Yes/No) of the order.
    pub side: Side,
    /// Ticker of the market associated with the order.
    pub ticker: String,
    /// Type of the order (e.g., market, limit).
    pub input_type: OrderType,
    /// The maximum cost the buyer is willing to incur for a 'buy' action. Optional.
    pub buy_max_cost: Option<i64>,
    /// Expiration time of the order. Optional.
    pub expiration_ts: Option<i64>,
    /// Price of the 'No' option in the order. Optional.
    pub no_price: Option<i64>,
    /// The minimum position the seller is willing to hold after selling. Optional.
    pub sell_position_floor: Option<i32>,
    /// Price of the 'Yes' option in the order. Optional.
    pub yes_price: Option<i64>,
}

impl OrderParams for OrderCreationField {
    fn get_params(
        self,
    ) -> (
        Action,
        Option<String>,
        i32,
        Side,
        String,
        OrderType,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i32>,
        Option<i64>,
    ) {
        (
            self.action,
            self.client_order_id,
            self.count,
            self.side,
            self.ticker,
            self.input_type,
            self.buy_max_cost,
            self.expiration_ts,
            self.no_price,
            self.sell_position_floor,
            self.yes_price,
        )
    }
}

/// The side of a market position in the Kalshi exchange.
///
/// This enum is used to indicate whether a market position, order, or trade is associated with the 'Yes' or 'No' outcome of a market event.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    /// Represents a position, order, or trade associated with the 'Yes' outcome of a market event.
    Yes,
    /// Represents a position, order, or trade associated with the 'No' outcome of a market event.
    No,
}

/// This enum is used to specify the type of action a user wants to take in an order, either buying or selling.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    /// Represents a buy action.
    Buy,
    /// Represents a sell action.
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

/// The status of an order in the Kalshi exchange.
///
/// This enum categorizes an order's lifecycle state, from creation to completion or cancellation.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    /// The order is active but not yet filled or partially filled and still in the order book.
    Resting,
    /// The order has been canceled and is no longer active.
    Canceled,
    /// The order has been fully executed.
    Executed,
    /// The order has been created and is awaiting further processing.
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

/// Defines the type of an order in the Kalshi exchange.
///
/// This enum is used to specify the nature of the order, particularly how it interacts with the market.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    /// A market order is executed immediately at the current market price.
    Market,
    /// A limit order is set to be executed at a specific price or better.
    Limit,
}

#[allow(dead_code)]
trait OrderParams {
    fn get_params(
        self,
    ) -> (
        Action,
        Option<String>,
        i32,
        Side,
        String,
        OrderType,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i32>,
        Option<i64>,
    );
}

impl OrderParams
    for (
        Action,
        Option<String>,
        i32,
        Side,
        String,
        OrderType,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i32>,
        Option<i64>,
    )
{
    fn get_params(
        self,
    ) -> (
        Action,
        Option<String>,
        i32,
        Side,
        String,
        OrderType,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i32>,
        Option<i64>,
    ) {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::portfolio::MultipleOrderResponse;

    #[test]
    fn test_serialize_multiple_order_response() -> serde_json::Result<()> {
        let json = r#"{"orders":[],"cursor":""}"#;
        let result = serde_json::from_str::<MultipleOrderResponse>(json)?;
        assert!(result.orders.is_empty());
        assert!(result.cursor.is_none());
        Ok(())
    }
}
