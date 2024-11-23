use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl<State> Kalshi<State> {
    /// Asynchronously retrieves the current status of the exchange.
    ///
    /// This function makes an HTTP GET request to the Kalshi exchange status endpoint
    /// and returns the current status of the exchange, including whether trading
    /// and the exchange itself are active.
    ///
    /// # Returns
    /// - `Ok(ExchangeStatus)`: ExchangeStatus object on successful retrieval.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    /// ```
    /// kalshi_instance.get_exchange_status().await.unwrap();
    /// ```
    pub async fn get_exchange_status(&self) -> Result<ExchangeStatus, KalshiError> {
        let exchange_status_url: &str = &format!("{}/exchange/status", self.base_url);

        let result: ExchangeStatus = self
            .client
            .get(exchange_status_url)
            .send()
            .await?
            .json()
            .await?;

        Ok(result)
    }

    /// Asynchronously retrieves the exchange's trading schedule.
    ///
    /// Sends a GET request to the Kalshi exchange schedule endpoint to obtain
    /// detailed schedule information, including standard trading hours and
    /// maintenance windows.
    ///
    /// # Returns
    /// - `Ok(ExchangeScheduleStandard)`: ExchangeScheduleStandard object on success.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    /// ```
    /// kalshi_instance.get_exchange_schedule().await.unwrap();
    /// ```
    pub async fn get_exchange_schedule(&self) -> Result<ExchangeScheduleStandard, KalshiError> {
        let exchange_schedule_url: &str = &format!("{}/exchange/schedule", self.base_url);

        let result: ExchangeScheduleResponse = self
            .client
            .get(exchange_schedule_url)
            .send()
            .await?
            .json()
            .await?;
        Ok(result.schedule)
    }
}

/// Represents the standard trading hours and maintenance windows of the exchange.
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeScheduleStandard {
    pub standard_hours: StandardHours,
    pub maintenance_windows: Vec<String>,
}

/// Internal struct used for deserializing the response from the exchange schedule endpoint.
#[derive(Debug, Deserialize, Serialize)]
struct ExchangeScheduleResponse {
    schedule: ExchangeScheduleStandard,
}

/// Represents the status of the exchange, including trading and exchange activity.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeStatus {
    pub trading_active: bool,
    pub exchange_active: bool,
}

/// Contains the daily schedule for each day of the week.
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

/// Represents the opening and closing times of the exchange for a single day.
#[derive(Debug, Deserialize, Serialize)]
pub struct DaySchedule {
    pub open_time: String,
    pub close_time: String,
}
