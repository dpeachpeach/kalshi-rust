
#[macro_use] 
mod utils;
pub mod kalshi_error;
pub mod auth;
pub mod exchange;
pub mod portfolio;
pub mod market; 

// imports
use reqwest;
use serde::{Deserialize, Serialize};

// Main Implementation, plan to abstract out in the future
#[derive(Debug)]
pub struct Kalshi<'a> {
    base_url: &'a str,
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

    pub fn get_user_token(&self) -> Option<String> {
        match &self.curr_token {
            Some(val) => return Some(val.clone()),
            _ => return None,
        }
    }




    
}

// GENERAL ENUMS
// -----------------------------------------------
pub enum TradingEnvironment {
    DemoMode,
    LiveMarketMode,
}



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Resting,
    Cancelled,
    Executed,
    Pending,
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
