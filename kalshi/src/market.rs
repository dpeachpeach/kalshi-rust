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
}
