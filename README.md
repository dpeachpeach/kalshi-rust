# kalshi-rust

## Rust Wrapper for the Kalshi trading API

This is a wrapper for the [Kalshi](https://kalshi.com/) trading API written by
and for those using Rust. This wrapper is asynchronous and typically more
performant than the official Python API provided by the developers, presented
here: [_KalshiDevAPI_](https://github.com/Kalshi/kalshi-python).

You can authenticate using email/password or api key (even for websockets).

## Documentation

Read the fully-featured docs [here](https://docs.rs/kalshi/0.9.0/kalshi/) and
check the project out on [crates.io](https://crates.io/crates/kalshi/0.9.0).

## Sample Bot

The Sample Bot directory is an example script that completes all the tasks
required to obtain advanced API access from the developers.

## Featurelist + Roadmap

### HTTP Requests: ✅

As of now the project supports interacting with Kalshi's RESTful API **fully**.

However any user of this library can build a rust trading bot using this library
if they wish!

Every function present in the library wraps around the
[Kalshi Trading API](https://trading-api.readme.io/reference/getting-started).

#### HTTP Feature Table

| Feature                               | Description                                             | Status |
| ------------------------------------- | ------------------------------------------------------- | ------ |
| **Auth/Login**                        | Retreiving your user token                              | ✅     |
| **Auth/Logout**                       | Deleting your user token                                | ✅     |
| **Exchange/GetSchedule**              | Retrieve Exchange Schedule                              | ✅     |
| **Exchange/GetExchangeStatus**        | Retreive Exchange Status                                | ✅     |
| **Portfolio/GetBalance**              | Get User Balance                                        | ✅     |
| **Portfolio/GetFills**                | Get User's Fills that fit certain criteria              | ✅     |
| **Portfolio/GetOrders**               | Get User's orders that fit certain criteria             | ✅     |
| **Portfolio/CreateOrder**             | Submit an Order                                         | ✅     |
| **Portfolio/BatchCreateOrders**       | Submit multiple Orders                                  | ❌     |
| **Portfolio/BatchCancelOrders**       | Cancel Multiple Orders (Advanced Users Only)            | ✅     |
| **Portfolio/GetOrder**                | Get a single Order                                      | ✅     |
| **Portfolio/CancelOrder**             | Cancel an order                                         | ✅     |
| **Portfolio/DecreaseOrder**           | Decrease Order amount                                   | ✅     |
| **Portfolio/GetPositions**            | Get Positions (Get all the positions of logged in user) | ✅     |
| **Portfolio/GetPortfolioSettlements** | Get Portfolio Settlements (Get settlement history)      | ✅     |
| **Market/GetEvents**                  | Get data about all events                               | ✅     |
| **Market/GetEvent**                   | Get data about a single event                           | ✅     |
| **Market/GetMarkets**                 | Get data about all markets                              | ✅     |
| **Market/GetTrades**                  | Get data about trades fitting certain criteria          | ✅     |
| **Market/GetMarket**                  | Get data about a single market                          | ✅     |
| **Market/GetMarketHistory**           | Get data about a single market's historical data        | ✅     |
| **Market/GetMarketOrderBook**         | Get a market's order book                               | ✅     |
| **Market/GetSeries**                  | Get data about a series                                 | ✅     |

### Websocket Requests: ⌛

Websocket's are a work in progress

See documentation here
[Kalshi Trading API](https://trading-api.readme.io/reference/introduction).

#### Websocket Feature Table

| Command                | Description                       | Status |
| ---------------------- | --------------------------------- | ------ |
| **Subscribe**          | Subscribe to one or many channels | ✅     |
| **Unsubscribe**        | Cancel one or more subscriptions  | ✅     |
| **UpdateSubscription** | Updates an existing subscription  | ✅     |

| Channels             | Description                                                                                                                                                            | Status |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ |
| **Orderbook**        | A complete view of the order book's aggregated price levels on a given market and all further updates to it.                                                           | ✅     |
| **Ticker**           | The list price ticker for a given market.                                                                                                                              | ✅     |
| **Trade**            | Update the client with the most recent trades that occur in the markets that the client is interested on.                                                              | ✅     |
| **Fill**             | Update the client with the most recent fills, that means trades in that occur in the markets that the client is interested on.                                         | ✅     |
| **Market Lifecycle** | Update the client with new market lifecycle events of the following types: open, pause, close, determination and settlement with corresponding details for each event. | ✅     |

## Troubleshooting & Undefined behavior

The Kalshi API is under-documented and rapidly changing

If you are using 2FA you may need to use an API key versus email/password

If you signed up with SSO (Apple or Google) on Kalshi and you need
email/password login, you can use the reset password flow with your SSO email to
get valid credentials for the API.
