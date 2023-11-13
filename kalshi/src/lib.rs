//! An HTTPS and Websocket wrapper that allows users to write trading bots for the Kalshi events trading platform.
//! 
//! kalshi-rust is asynchronous, performant, and succint. Dash past verbose and annoying HTTPS requests
//! and use this wrapper to quickly write blazingly fast trading bots!
//! 
//! kalshi-rust is written for the [Kalshi events trading platform](https://kalshi.com). 
//! As of version 0.9.0, HTTPS features are fully complete but websocket support and advanced API access features are not complete. 
//! If you'd like to keep up on kalshi-rust's development and view a sample trading script, 
//! feel free to visit the [github](https://github.com/dpeachpeach/kalshi-rust) and drop a star!.
//! I'm a student developer and any recognition is incredibly helpful!
//!
//! ## The Kalshi Struct
//!
//! The [Kalshi](Kalshi) struct is the central component of this crate. 
//! All authentication, order routing, market requests, and position snapshots are handled through the struct.
//! 
//! 
//!
//! For more details, see [Kalshi](Kalshi).
//!
//! ## Other Features
//! [brief overview of other parts of the crate]

#[macro_use] 
mod utils;
mod kalshi_error;
mod auth;
mod exchange;
mod portfolio;
mod market; 

pub use kalshi_error::*;
pub use auth::*;
pub use exchange::*;
pub use portfolio::*;
pub use market::*;

// imports
use reqwest;

/// Main struct for handling [specific functionality].
///
/// The `Kalshi` struct is used to [description of what it does].
///
/// # Examples
///
/// ```
/// use my_crate::Kalshi;
///
/// let kalshi = Kalshi::new();
/// // Example usage
/// ```
#[derive(Debug)]
pub struct Kalshi<'a> {
    base_url: &'a str,
    curr_token: Option<String>,
    member_id: Option<String>,
    client: reqwest::Client,
}

impl<'a> Kalshi<'a> {
    pub fn new(trading_env: TradingEnvironment) -> Kalshi<'a> {
        return Kalshi {
            base_url: utils::build_base_url(trading_env) ,
            curr_token: None,
            member_id: None,
            client: reqwest::Client::new(),
        };
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

// unit tests, absent at the moment. all test logic is handled in the test bot dir
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
