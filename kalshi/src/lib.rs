const LOGOUT_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/logout";
// imports
use reqwest;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Main Implementation, plan to abstract out in the future
#[derive(Debug)]
pub struct Kalshi {
    logged_in: bool,
    curr_token: Option<String>,
    member_id: Option<String>,
    client: reqwest::Client
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    member_id: String, 
    token: String
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginPayload {
    email: String, 
    password: String
}

impl Kalshi {

    pub fn new() -> Kalshi {
        return Kalshi{
            logged_in: false,
            curr_token: None,
            member_id: None,
            client: reqwest::Client::new()
        };
    }

    pub async fn login(&mut self, user: &str, password: &str) -> Result<(), reqwest::Error> {
        const LOGIN_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/login";

        let login_payload = LoginPayload {
            email: user.to_string(),
            password: password.to_string(),
        };
    
        let result: LoginResponse =  self.client
                                        .post(LOGIN_URL)
                                        .json(&login_payload)
                                        .send()
                                        .await?
                                        .json()
                                        .await?;
        
        self.curr_token = Some(result.token);
        self.member_id = Some(result.member_id);
        self.logged_in = true;

        return Ok(())
    }

    pub async fn logout(&self) -> Result<(), Box<dyn std::error::Error>> {
        const LOGOUT_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/logout";
        return Ok(()) 
    }

    pub fn get_user_token(&self) -> Option<String> {
        match &self.curr_token {
            Some(val) => return Some(val.clone()),
            _ => return None
        }
    }

}


// Enums
pub enum ConnectionError {
    ClientConnectionFailure,
    IncorrectCredentials,
    ServerConnectionFailure
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
