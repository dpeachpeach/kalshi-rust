use super::Kalshi;

use serde::{Deserialize, Serialize};

impl<'a> Kalshi<'a> {
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
}

// used in get_exchange_schedule
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeScheduleStandard {
    pub standard_hours: StandardHours,
    pub maintenance_windows: Vec<String>,
}

// used in get_exchange_schedule
#[derive(Debug, Deserialize, Serialize)]
struct ExchangeScheduleResponse {
    schedule: ExchangeScheduleStandard,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeStatus {
    pub trading_active: bool,
    pub exchange_active: bool,
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
pub struct DaySchedule {
    pub open_time: String,
    pub close_time: String,
}
