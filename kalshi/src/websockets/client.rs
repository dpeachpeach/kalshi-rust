#![allow(unused)]

use futures_util::{select_biased, FutureExt, SinkExt, Stream, StreamExt};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::{error::Error, str::FromStr, time::Duration, vec};
use tokio::{
    net::TcpStream,
    sync::{
        broadcast::{channel, Receiver, Sender},
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    },
    task::JoinHandle,
    time::{interval, MissedTickBehavior},
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        client::IntoClientRequest,
        handshake,
        http::{HeaderMap, HeaderValue, Request, Uri},
        Message,
    },
    MaybeTlsStream, WebSocketStream,
};

use crate::{utils::api_key_headers, Kalshi, KalshiAuth};

use super::{
    commands::{
        KalshiCommand, KalshiSubscribeCommandParams, KalshiUnsubscribeCommandParams,
        KalshiUpdateSubscriptionAction, KalshiUpdateSubscriptionCommandParams,
    },
    responses::KalshiWebsocketResponse,
    KalshiChannel,
};

#[derive(Clone, Debug)]
pub enum KalshiWebsocketError {
    WebSocketError(String),
    SerializationError(String),
    ConnectionClosed,
}

impl std::fmt::Display for KalshiWebsocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KalshiWebsocketError::WebSocketError(msg) => write!(f, "WebSocket error: {}", msg),
            KalshiWebsocketError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
            KalshiWebsocketError::ConnectionClosed => write!(f, "Connection closed"),
        }
    }
}

impl std::error::Error for KalshiWebsocketError {}

pub struct KalshiWebsocketClient {
    _ws: JoinHandle<()>,
    next_cmd_id: u32,
    to_kalshi: UnboundedSender<KalshiCommand>,
    from_kalshi: Receiver<Result<KalshiWebsocketResponse, KalshiWebsocketError>>,
}

impl Kalshi {
    pub async fn connect_ws(&mut self) -> Result<KalshiWebsocketClient, Box<dyn Error>> {
        KalshiWebsocketClient::connect(self).await
    }

    pub fn get_ws_url(&self) -> &str {
        &self.ws_url
    }
}

impl<'a> KalshiWebsocketClient {
    pub async fn connect(kalshi: &mut Kalshi) -> Result<Self, Box<dyn Error>> {
        let mut req = Uri::from_str(kalshi.get_ws_url())?.into_client_request()?;
        let mut headers = req.headers_mut();
        match &mut kalshi.auth {
            KalshiAuth::EmailPassword => {
                let curr_token = kalshi
                    .get_user_token()
                    .ok_or("No user token, login first using .login(..)".to_string())?;
                headers.insert("Authorization", HeaderValue::from_str(&curr_token)?);
            }
            KalshiAuth::ApiKey { key_id, signer, .. } => {
                let api_key_headers =
                    api_key_headers(key_id, signer, "/trade-api/ws/v2", Method::GET)?;
                for (key, val) in api_key_headers {
                    headers.insert(key, HeaderValue::from_str(val.as_str())?);
                }
            }
        }
        let req_clone = req.clone();
        let (ws_stream, res) = connect_async(req).await.inspect_err(|e| match e {
            tokio_tungstenite::tungstenite::Error::Http(res) => {
                if let Some(body) = res.body() {
                    if let Ok(error_body) = String::from_utf8(body.to_vec()) {
                        eprintln!("Request was {:?}", req_clone);
                        eprintln!("Kalshi error response was {}", error_body);
                    }
                }
            }
            _ => {}
        })?;

        let (to_kalshi_tx, to_kalshi_rx) = unbounded_channel::<KalshiCommand>();
        let (from_kalshi_tx, from_kalshi_rx) =
            channel::<Result<KalshiWebsocketResponse, KalshiWebsocketError>>(1024);

        let _ws = tokio::spawn(kalshi_ws_handler(ws_stream, from_kalshi_tx, to_kalshi_rx));

        Ok(KalshiWebsocketClient {
            next_cmd_id: 1,
            to_kalshi: to_kalshi_tx,
            from_kalshi: from_kalshi_rx,
            _ws,
        })
    }

    /// Subscribe to one or more channels on one, more, or all markets (all markets mode not available for orderbook delta)
    /// You will receive an error if you resubscribe to the same channel/market combinations
    ///
    /// # Returns
    ///
    /// Returns a `Result<u32, Box<dyn Error>>` where the unsigned integer is the command id.
    ///
    /// ```
    ///
    pub async fn subscribe(
        &mut self,
        channels: Vec<KalshiChannel>,
        market_tickers: Vec<String>,
    ) -> Result<u32, Box<dyn Error>> {
        let cmd_id = self.next_cmd_id;
        if channels.contains(&KalshiChannel::OrderbookDelta) && market_tickers.len() == 0 {
            return Err("Cannot subscribe to orderbook deltas for all market tickers, provide at least one market ticker".to_string().into());
        }
        let msg = KalshiCommand::Subscribe {
            id: cmd_id,
            params: KalshiSubscribeCommandParams {
                channels,
                market_tickers,
            },
        };
        self.to_kalshi.send(msg)?;
        self.next_cmd_id += 1;
        Ok(cmd_id)
    }

    /// Unsubscribe one or more existing subscriptions
    ///
    /// # Returns
    ///
    /// Returns a `Result<u32, Box<dyn Error>>` where the unsigned integer is the command id
    ///
    /// ```
    ///
    pub async fn unsubscribe(&mut self, sids: Vec<u32>) -> Result<u32, Box<dyn Error>> {
        let cmd_id = self.next_cmd_id;
        let msg = KalshiCommand::Unsubscribe {
            id: cmd_id,
            params: KalshiUnsubscribeCommandParams { sids },
        };
        self.to_kalshi.send(msg)?;
        self.next_cmd_id += 1;
        Ok(cmd_id)
    }

    /// Add or delete markets on an existing subscription
    ///
    /// # Returns
    ///
    /// Returns a `Result<u32, Box<dyn Error>>` where the unsigned integer is the command id
    ///
    /// ```
    ///
    pub async fn update_subscription(
        &mut self,
        sid: u32,
        market_tickers: Vec<String>,
        action: KalshiUpdateSubscriptionAction,
    ) -> Result<u32, Box<dyn Error>> {
        let cmd_id = self.next_cmd_id;
        let msg = KalshiCommand::UpdateSubscription {
            id: cmd_id,
            params: KalshiUpdateSubscriptionCommandParams {
                market_tickers,
                action,
                sids: [sid],
            },
        };
        self.to_kalshi.send(msg)?;
        self.next_cmd_id += 1;
        Ok(cmd_id)
    }

    /// Get a broadcast receiver from the websocket stream
    /// You probably want to use `.stream()`
    ///
    /// # Returns
    ///
    /// Returns a broadcast::Receiver<KalshiWebsocketResponse>
    ///
    /// ```
    ///
    pub fn receiver(&self) -> Receiver<Result<KalshiWebsocketResponse, KalshiWebsocketError>> {
        self.from_kalshi.resubscribe()
    }

    /// Gracefully closes the websocket connection consuming the client
    ///
    /// ```
    ///
    fn close(self) -> Result<(), Box<dyn Error>> {
        self.to_kalshi.send(KalshiCommand::End)?;
        Ok(())
    }
}

async fn kalshi_ws_handler(
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    from_kalshi_tx: Sender<Result<KalshiWebsocketResponse, KalshiWebsocketError>>,
    mut to_kalshi_rx: UnboundedReceiver<KalshiCommand>,
) {
    let mut stream = Box::pin(stream.fuse());
    let mut heartbeat = interval(Duration::from_secs(10));
    heartbeat.set_missed_tick_behavior(MissedTickBehavior::Skip);

    'out: loop {
        select_biased! {
            cmd = to_kalshi_rx.recv().fuse() => {
                match cmd {
                    Some(cmd) => {
                        match serde_json::to_string(&cmd) {
                            Ok(msg) => {
                                stream.send(Message::text(msg)).await.unwrap();
                            },
                            Err(e) => {
                                from_kalshi_tx.send(Err(KalshiWebsocketError::SerializationError(e.to_string())));
                            }
                        }

                    },
                    _ => {
                        from_kalshi_tx.send(Err(KalshiWebsocketError::ConnectionClosed));
                        break 'out;
                    }
                }
            }
            _ = heartbeat.tick().fuse() => {
                match stream.send(Message::Ping(vec![])).await {
                    Err(e) => {
                        from_kalshi_tx.send(Err(KalshiWebsocketError::WebSocketError(e.to_string())));
                    },
                    _ => {}
                }
            }
            item = stream.select_next_some() => {
                match item {
                    Ok(msg) => {
                        match msg {
                            Message::Text(text) => {
                                match serde_json::from_str::<KalshiWebsocketResponse>(&text) {
                                    Ok(res) => from_kalshi_tx.send(Ok(res)),
                                    Err(e) => from_kalshi_tx.send(Err(KalshiWebsocketError::SerializationError(e.to_string()))),
                                };
                            },
                            Message::Close(_) => {
                                from_kalshi_tx.send(Err(KalshiWebsocketError::ConnectionClosed));
                                break 'out;
                            }
                            // Pings should be automatically handled by tokio_tungstenite
                            // All other messages are unhandled
                            _ => {}
                        }
                    },
                    Err(e) => {
                       from_kalshi_tx.send(Err(KalshiWebsocketError::WebSocketError(e.to_string())));
                    }
                }
            }
        }
    }
}
