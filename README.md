# kalshi-rust

## Rust Wrapper for the Kalshi trading API

This is a wrapper for the [Kalshi](https://kalshi.com/) trading API written by and for those using Rust. 
This wrapper is asynchronous and typically more performant than the official Python API provided by the developers, presented here: [*KalshiDevAPI*](https://github.com/Kalshi/kalshi-python).


## Sample Bot

The Sample Bot directory is an example script that completes all the tasks required to obtain advanced API access from the developers.

## Featurelist + Roadmap

### HTTP Requests: ✅ 
As of now the project supports interacting with Kalshi's RESTful API **fully**.
The only features missing from that part of the project are as follows:
- Advanced API features (Waiting for the devs to give me access)

However any user of this library can build a rust trading bot using this library 
if they wish!

Every function present in the library wraps around the [Kalshi Trading API](https://trading-api.readme.io/reference/getting-started).

#### HTTP Feature Table

| Feature                | Description                           | Status      |
|------------------------|---------------------------------------|-------------|
| **Auth/Login**          | Retreiving your user token       |  ✅         |
| **Auth/Logout**         | Deleting your user token        |    ✅     |
| **Exchange/GetSchedule**          | Retrieve Exchange Schedule     |   ✅    |
| **Exchange/GetExchangeStatus**          | Retreive Exchange Status   |   ✅        |
| **Portfolio/GetBalance** | Get User Balance |     ✅  |
| **Portfolio/GetFills** | Get User's Fills that fit certain criteria|  ✅        |
| **Portfolio/GetOrders** | Get User's orders that fit certain criteria |  ✅       |
| **Portfolio/CreateOrder** | Submit an Order |✅         |
| **Portfolio/BatchCreateOrders** | Submit multiple Orders |❌          |
| **Portfolio/BatchCancelOrders** | Cancel Multiple Orders (Advanced Users Only) |❌          |
| **Portfolio/GetOrder** | Get a single Order | ✅          |
| **Portfolio/CancelOrder** | Cancel an order |✅          |
| **Portfolio/DecreaseOrder** | Decrease Order amount |✅          |
| **Portfolio/GetPositions** | Get Positions (Get all the positions of logged in user) |✅           |
| **Portfolio/GetPortfolioSettlements** | Get Portfolio Settlements (Get settlement history) |✅         |
| **Market/GetEvents** | Get data about all events |✅         |
| **Market/GetEvent** | Get data about a single event |✅         |
| **Market/GetMarkets** | Get data about all markets |✅       |
| **Market/GetTrades** | Get data about trades fitting certain criteria |✅           |
| **Market/GetMarket** | Get data about a single market |✅          |
| **Market/GetMarketHistory** | Get data about a single market's historical data |✅           |
| **Market/GetMarketOrderBook** | Get a market's order book |✅         |
| **Market/GetSeries** | Get data about a series |✅         |

### Writing detailed docs:  🟡 
I'm writing detailed docs + a user manual  to support a user's implementation of a Kalshi bot.









