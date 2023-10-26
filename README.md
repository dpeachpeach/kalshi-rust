# kalshi-rust

## Rust Wrapper for the Kalshi trading API

This is a wrapper for the [Kalshi](https://kalshi.com/) trading API written by and for those using Rust. 

## Featurelist / Roadmap

### WIP Statement
As of now this project is a **Work in Progress**, this means the feature list is not complete. Anyone is welcome to try out the API as they see fit. A sample implementation is present in the *test_dir* directory for testing and I will develop a full arbitrage 'sample bot' at the conclusion of the project for anyone to edit / mess around with should they want to start using the API.

### Project Goals
1. Complete every function that wraps around the [Kalshi Trading API}(https://trading-api.readme.io/reference/getting-started).
2. Work on an implementation for websockets (Stretch goal at the moment).
3. Write detailed documentation.
4. Write sample_bot that utilizes basic price arbitrage.

## Feature Roadmap

| Feature                | Description                           | Status      |
|------------------------|---------------------------------------|-------------|
| **Auth/Login**          | Retreiving your user token       |  ✅         |
| **Auth/Logout**         | Deleting your user token        |    ✅     |
| **Exchange/GetSchedule**          | Retrieve Exchange Schedule     |   ✅    |
| **Exchange/GetExchangeStatus**          | Retreive Exchange Status   |   ✅        |
| **Portfolio/GetBalance** | Get User Balance |     ✅  |
| **Portfolio/GetFills** | Get All of User Fill's| 🟡       |
| **Portfolio/GetOrders** | Get All of User's orders | 🟡       |
| **Portfolio/CreateOrder** | Submit an Order |❌          |
| **Portfolio/BatchCreateOrders** | Submit multiple Orders |❌          |
| **Portfolio/BatchCancelOrders** | Cancel Multiple Orders (Advanced Users Only) |❌          |
| **Portfolio/GetOrder** | Get a single Order |❌          |
| **Portfolio/CancelOrder** | Cancel an order |❌          |
| **Portfolio/DecreaseOrder** | Decrease Order amount |❌          |
| **Portfolio/GetPositions** | Get Positions (Get all the positions of logged in user) |❌          |
| **Portfolio/GetPortfolioSettlements** | Get Portfolio Settlements (Get settlement history) |❌          |
| **Market/GetEvents** | Get data about all events |❌          |
| **Market/GetEvent** | Get data about a single event |🟡         |
| **Market/GetMarkets** | Get data about all markets |❌          |
| **Market/GetTrades** | Get data about all trades |❌          |
| **Market/GetMarket** | Get data about a single market |🟡          |
| **Market/GetMarketHistory** | Get data about a single market's historical data |❌          |
| **Market/GetMarketOrderBook** | Get a market's order book |🟡         |
| **Market/GetSeries** | Get data about a series |❌          |






