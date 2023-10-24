use futures::TryFutureExt;
use kalshi::Kalshi;
use std::env;
use dotenv::dotenv;
        
extern crate kalshi;

#[tokio::main]
async fn main() {
    dotenv().ok();
    

    let mut username = "dummy".to_string();
    let mut password = "dummy".to_string();
    
    if let Ok(key) = env::var("PASSWORD") {
        println!("got password");
        password = key;
    }
    if let Ok(user) = env::var("USER_NAME") {
        println!("got user");
        username = user;
    }
    // main testing logic, ignoring unit tests for now
    let mut kalshi_instance = Kalshi::new();
    kalshi_instance.login(&username, &password).await;
    let token = kalshi_instance.get_user_token().unwrap();
    println!("{}",token);
    let balance = kalshi_instance.get_balance().await.unwrap();
    println!("{}", balance);
}
