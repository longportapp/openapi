use std::sync::Arc;

use longport_httpcli::{HttpClient, Json, Method};
use longport_wscli::WsClientError;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Result, serde_utils,
    trade::{
        AccountBalance, CashFlow, EstimateMaxPurchaseQuantityOptions, Execution,
        FundPositionsResponse, GetCashFlowOptions, GetFundPositionsOptions,
        GetHistoryExecutionsOptions, GetHistoryOrdersOptions, GetStockPositionsOptions,
        GetTodayExecutionsOptions, GetTodayOrdersOptions, MarginRatio, Order, OrderDetail,
        PushEvent, ReplaceOrderOptions, StockPositionsResponse, SubmitOrderOptions, TopicType,
        core::{Command, Core},
    },
};

#[derive(Debug, Deserialize)]
struct EmptyResponse {}

/// Response for submit order request
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitOrderResponse {
    /// Order id
    pub order_id: String,
}

/// Response for estimate maximum purchase quantity
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateMaxPurchaseQuantityResponse {
    /// Cash available quantity
    #[serde(with = "serde_utils::decimal_empty_is_0")]
    pub cash_max_qty: Decimal,
    /// Margin available quantity
    #[serde(with = "serde_utils::decimal_empty_is_0")]
    pub margin_max_qty: Decimal,
}

struct InnerTradeContext {
    command_tx: mpsc::UnboundedSender<Command>,
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerTradeContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("trade context dropped");
        });
    }
}

/// Trade context
#[derive(Clone)]
pub struct TradeContext(Arc<InnerTradeContext>);

impl TradeContext {
    /// Create a `TradeContext`
    pub async fn try_new(
        config: Arc<Config>,
    ) -> Result<(Self, mpsc::UnboundedReceiver<PushEvent>)> {
        let log_subscriber = config.create_log_subscriber("trade");

        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating trade context");
        });

        let http_cli = config.create_http_client();
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (push_tx, push_rx) = mpsc::unbounded_channel();
        let core = Core::try_new(config, command_rx, push_tx)
            .with_subscriber(log_subscriber.clone())
            .await?;
        tokio::spawn(core.run().with_subscriber(log_subscriber.clone()));

        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!("trade context created");
        });

        Ok((
            TradeContext(Arc::new(InnerTradeContext {
                http_cli,
                command_tx,
                log_subscriber,
            })),
            push_rx,
        ))
    }

    /// Returns the log subscriber
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    /// Subscribe
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/trade-push#subscribe>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config, decimal,
    ///     trade::{OrderSide, OrderType, SubmitOrderOptions, TimeInForceType, TradeContext},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, mut receiver) = TradeContext::try_new(config).await?;
    ///
    /// let opts = SubmitOrderOptions::new(
    ///     "700.HK",
    ///     OrderType::LO,
    ///     OrderSide::Buy,
    ///     decimal!(200),
    ///     TimeInForceType::Day,
    /// )
    /// .submitted_price(decimal!(50i32));
    /// let resp = ctx.submit_order(opts).await?;
    /// println!("{:?}", resp);
    ///
    /// while let Some(event) = receiver.recv().await {
    ///     println!("{:?}", event);
    /// }
    ///
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn subscribe<I>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = TopicType>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::Subscribe {
                topics: topics.into_iter().collect(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Unsubscribe
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/trade-push#cancel-subscribe>
    pub async fn unsubscribe<I>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = TopicType>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::Unsubscribe {
                topics: topics.into_iter().collect(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Get history executions
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/execution/history_executions>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     trade::{GetHistoryExecutionsOptions, TradeContext},
    ///     Config,
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let opts = GetHistoryExecutionsOptions::new()
    ///     .symbol("700.HK")
    ///     .start_at(datetime!(2022-05-09 0:00 UTC))
    ///     .end_at(datetime!(2022-05-12 0:00 UTC));
    /// let resp = ctx.history_executions(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn history_executions(
        &self,
        options: impl Into<Option<GetHistoryExecutionsOptions>>,
    ) -> Result<Vec<Execution>> {
        #[derive(Deserialize)]
        struct Response {
            trades: Vec<Execution>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/trade/execution/history")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .trades)
    }

    /// Get today executions
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/execution/today_executions>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config,
    ///     trade::{GetTodayExecutionsOptions, TradeContext},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let opts = GetTodayExecutionsOptions::new().symbol("700.HK");
    /// let resp = ctx.today_executions(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn today_executions(
        &self,
        options: impl Into<Option<GetTodayExecutionsOptions>>,
    ) -> Result<Vec<Execution>> {
        #[derive(Deserialize)]
        struct Response {
            trades: Vec<Execution>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/trade/execution/today")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .trades)
    }

    /// Get history orders
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/order/history_orders>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     trade::{GetHistoryOrdersOptions, OrderSide, OrderStatus, TradeContext},
    ///     Config, Market,
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let opts = GetHistoryOrdersOptions::new()
    ///     .symbol("700.HK")
    ///     .status([OrderStatus::Filled, OrderStatus::New])
    ///     .side(OrderSide::Buy)
    ///     .market(Market::HK)
    ///     .start_at(datetime!(2022-05-09 0:00 UTC))
    ///     .end_at(datetime!(2022-05-12 0:00 UTC));
    /// let resp = ctx.history_orders(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn history_orders(
        &self,
        options: impl Into<Option<GetHistoryOrdersOptions>>,
    ) -> Result<Vec<Order>> {
        #[derive(Deserialize)]
        struct Response {
            orders: Vec<Order>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/trade/order/history")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .orders)
    }

    /// Get today orders
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/order/today_orders>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config, Market,
    ///     trade::{GetTodayOrdersOptions, OrderSide, OrderStatus, TradeContext},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let opts = GetTodayOrdersOptions::new()
    ///     .symbol("700.HK")
    ///     .status([OrderStatus::Filled, OrderStatus::New])
    ///     .side(OrderSide::Buy)
    ///     .market(Market::HK);
    /// let resp = ctx.today_orders(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn today_orders(
        &self,
        options: impl Into<Option<GetTodayOrdersOptions>>,
    ) -> Result<Vec<Order>> {
        #[derive(Deserialize)]
        struct Response {
            orders: Vec<Order>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/trade/order/today")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .orders)
    }

    /// Replace order
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/order/replace>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config, decimal,
    ///     trade::{ReplaceOrderOptions, TradeContext},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let opts =
    ///     ReplaceOrderOptions::new("709043056541253632", decimal!(100)).price(decimal!(300i32));
    /// let resp = ctx.replace_order(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn replace_order(&self, options: ReplaceOrderOptions) -> Result<()> {
        Ok(self
            .0
            .http_cli
            .request(Method::PUT, "/v1/trade/order")
            .body(Json(options))
            .response::<Json<EmptyResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await
            .map(|_| ())?)
    }

    /// Submit order
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/order/submit>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config, decimal,
    ///     trade::{OrderSide, OrderType, SubmitOrderOptions, TimeInForceType, TradeContext},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let opts = SubmitOrderOptions::new(
    ///     "700.HK",
    ///     OrderType::LO,
    ///     OrderSide::Buy,
    ///     decimal!(200),
    ///     TimeInForceType::Day,
    /// )
    /// .submitted_price(decimal!(50i32));
    /// let resp = ctx.submit_order(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn submit_order(&self, options: SubmitOrderOptions) -> Result<SubmitOrderResponse> {
        let resp: SubmitOrderResponse = self
            .0
            .http_cli
            .request(Method::POST, "/v1/trade/order")
            .body(Json(options))
            .response::<Json<_>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0;
        _ = self.0.command_tx.send(Command::SubmittedOrder {
            order_id: resp.order_id.clone(),
        });
        Ok(resp)
    }

    /// Cancel order
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/order/withdraw>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, trade::TradeContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// ctx.cancel_order("709043056541253632").await?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn cancel_order(&self, order_id: impl Into<String>) -> Result<()> {
        #[derive(Debug, Serialize)]
        struct Request {
            order_id: String,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::DELETE, "/v1/trade/order")
            .response::<Json<EmptyResponse>>()
            .query_params(Request {
                order_id: order_id.into(),
            })
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await
            .map(|_| ())?)
    }

    /// Get account balance
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/asset/account>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, trade::TradeContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let resp = ctx.account_balance(None).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn account_balance(&self, currency: Option<&str>) -> Result<Vec<AccountBalance>> {
        #[derive(Debug, Serialize)]
        struct Request<'a> {
            currency: Option<&'a str>,
        }

        #[derive(Debug, Deserialize)]
        struct Response {
            list: Vec<AccountBalance>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/account")
            .query_params(Request { currency })
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .list)
    }

    /// Get cash flow
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/asset/cashflow>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     trade::{GetCashFlowOptions, TradeContext},
    ///     Config,
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let opts = GetCashFlowOptions::new(datetime!(2022-05-09 0:00 UTC), datetime!(2022-05-12 0:00 UTC));
    /// let resp = ctx.cash_flow(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn cash_flow(&self, options: GetCashFlowOptions) -> Result<Vec<CashFlow>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            list: Vec<CashFlow>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/cashflow")
            .query_params(options)
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .list)
    }

    /// Get fund positions
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/asset/fund>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, trade::TradeContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let resp = ctx.fund_positions(None).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn fund_positions(
        &self,
        opts: impl Into<Option<GetFundPositionsOptions>>,
    ) -> Result<FundPositionsResponse> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/fund")
            .query_params(opts.into().unwrap_or_default())
            .response::<Json<FundPositionsResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get stock positions
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/asset/stock>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, trade::TradeContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let resp = ctx.stock_positions(None).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn stock_positions(
        &self,
        opts: impl Into<Option<GetStockPositionsOptions>>,
    ) -> Result<StockPositionsResponse> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/stock")
            .query_params(opts.into().unwrap_or_default())
            .response::<Json<StockPositionsResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get margin ratio
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/asset/margin_ratio>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, trade::TradeContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let resp = ctx.margin_ratio("700.HK").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn margin_ratio(&self, symbol: impl Into<String>) -> Result<MarginRatio> {
        #[derive(Debug, Serialize)]
        struct Request {
            symbol: String,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/risk/margin-ratio")
            .query_params(Request {
                symbol: symbol.into(),
            })
            .response::<Json<MarginRatio>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get order detail
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/order/order_detail>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config, Market,
    ///     trade::{GetHistoryOrdersOptions, OrderSide, OrderStatus, TradeContext},
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let resp = ctx.order_detail("701276261045858304").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn order_detail(&self, order_id: impl Into<String>) -> Result<OrderDetail> {
        #[derive(Debug, Serialize)]
        struct Request {
            order_id: String,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/trade/order")
            .response::<Json<OrderDetail>>()
            .query_params(Request {
                order_id: order_id.into(),
            })
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Estimating the maximum purchase quantity for Hong Kong and US stocks,
    /// warrants, and options
    ///
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/order/estimate_available_buy_limit>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config,
    ///     trade::{EstimateMaxPurchaseQuantityOptions, OrderSide, OrderType, TradeContext},
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let resp = ctx
    ///     .estimate_max_purchase_quantity(EstimateMaxPurchaseQuantityOptions::new(
    ///         "700.HK",
    ///         OrderType::LO,
    ///         OrderSide::Buy,
    ///     ))
    ///     .await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn estimate_max_purchase_quantity(
        &self,
        opts: EstimateMaxPurchaseQuantityOptions,
    ) -> Result<EstimateMaxPurchaseQuantityResponse> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/trade/estimate/buy_limit")
            .query_params(opts)
            .response::<Json<EstimateMaxPurchaseQuantityResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }
}
