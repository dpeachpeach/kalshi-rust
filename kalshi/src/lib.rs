// imports
use reqwest;
use serde::{Serialize, Deserialize};

// Main Implementation, plan to abstract out in the future
#[derive(Debug)]
pub struct Kalshi<'a> {
    base_url: &'a str, 
    logged_in: bool,
    curr_token: Option<String>,
    member_id: Option<String>,
    client: reqwest::Client
}

impl<'a> Kalshi<'a> {

    pub fn new() -> Kalshi<'a> {
        return Kalshi {
            base_url: "",
            logged_in: false,
            curr_token: None,
            member_id: None,
            client: reqwest::Client::new()
        };
    }

    pub fn build_base_url(&mut self, trading_env: TradingEnvironment) -> (){
        match trading_env {
            TradingEnvironment::LiveMarketMode => {
                self.base_url = "https://trading-api.kalshi.com/trade-api/v2";
            },
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
    
        let result: LoginResponse =  self.client
                                        .post(login_url)
                                        .json(&login_payload)
                                        .send()
                                        .await?
                                        .json()
                                        .await?;

        self.curr_token = Some(format!("Bearer {}", result.token));
        self.member_id = Some(result.member_id);
        self.logged_in = true;

        return Ok(())
    }

    pub async fn logout(&self) -> Result<(), reqwest::Error> {

        //const LOGOUT_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/logout";
        
        let logout_url: &str = &format!("{}/logout", self.base_url.to_string());

        self.client
                .post(logout_url)
                .header("Authorization", self.curr_token.clone().unwrap())
                .header("content-type", "application/json".to_string())
                .send()
                .await?;

        return Ok(())
    }

    pub async fn get_balance(&self) -> Result<i64, reqwest::Error> {

        //const BALANCE_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/portfolio/balance";

        let balance_url: &str = &format!("{}/portfolio/balance", self.base_url.to_string()); 

        let result:BalanceResponse = self.client
                .get(balance_url)
                .header("Authorization", self.curr_token.clone().unwrap())
                .send()
                .await?
                .json()
                .await?;

        let bal = result.balance;
        
        return Ok(bal)
    }

    pub async fn get_exchange_status(&self) -> Result<ExchangeStatus, reqwest::Error> {
        todo!()
    }

    /* 
    pub async fn get_exchange_schedule(&self) -> Result<ExchangeSchedule, reqwest::Error> {
        todo!()
    }
    */

    pub fn get_user_token(&self) -> Option<String> {
        match &self.curr_token {
            Some(val) => return Some(val.clone()),
            _ => return None
        }
    }

    fn build_url(&self, endpoint: &str) -> String {
        todo!()
    }


}


// structs
// used in login method
#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    member_id: String, 
    token: String
}
// used in login method
#[derive(Debug, Serialize, Deserialize)]
struct LoginPayload {
    email: String, 
    password: String
}
// used in getbalance method
#[derive(Debug, Serialize, Deserialize)]
struct BalanceResponse {
    balance: i64 
}

// used in get_exchange_status
#[derive(Debug, Serialize, Deserialize)]
struct ExchangeStatus {
    trading_active: bool,
    exchange_active: bool,
}

// used in get_exchange_schedule
#[derive(Debug, Serialize, Deserialize)]

// Enums
pub enum TradingEnvironment {
    DemoMode,
    LiveMarketMode
}

// legacy code for now
pub enum ConnectionError {
    ClientConnectionFailure,
    IncorrectCredentials,
    ServerConnectionFailure
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
