use std::borrow::Borrow;

use longport::{Decimal, Market};
use longport_java_macros::impl_java_class;
use time::Date;

impl_java_class!(
    "com/longport/quote/Trade",
    longport::quote::Trade,
    [
        price,
        volume,
        timestamp,
        trade_type,
        direction,
        trade_session
    ]
);

impl_java_class!(
    "com/longport/quote/Brokers",
    longport::quote::Brokers,
    [
        position,
        #[java(priarray)]
        broker_ids
    ]
);

impl_java_class!(
    "com/longport/quote/Depth",
    longport::quote::Depth,
    [position, price, volume, order_num]
);

impl_java_class!(
    "com/longport/quote/Subscription",
    longport::quote::Subscription,
    [
        symbol,
        sub_types,
        #[java(objarray)]
        candlesticks
    ]
);

impl_java_class!(
    "com/longport/quote/PushQuote",
    longport::quote::PushQuote,
    [
        last_done,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status,
        trade_session,
        current_volume,
        current_turnover
    ]
);

impl_java_class!(
    "com/longport/quote/PushDepth",
    longport::quote::PushDepth,
    [
        #[java(objarray)]
        asks,
        #[java(objarray)]
        bids
    ]
);

impl_java_class!(
    "com/longport/quote/PushBrokers",
    longport::quote::PushBrokers,
    [
        #[java(objarray)]
        ask_brokers,
        #[java(objarray)]
        bid_brokers
    ]
);

impl_java_class!(
    "com/longport/quote/PushTrades",
    longport::quote::PushTrades,
    [
        #[java(objarray)]
        trades,
    ]
);

impl_java_class!(
    "com/longport/quote/PushCandlestick",
    longport::quote::PushCandlestick,
    [period, candlestick, is_confirmed]
);

impl_java_class!(
    "com/longport/quote/Security",
    longport::quote::Security,
    [symbol, name_cn, name_en, name_hk,]
);

impl_java_class!(
    "com/longport/quote/SecurityStaticInfo",
    longport::quote::SecurityStaticInfo,
    [
        symbol,
        name_cn,
        name_en,
        name_hk,
        exchange,
        currency,
        lot_size,
        total_shares,
        circulating_shares,
        hk_shares,
        eps,
        eps_ttm,
        bps,
        dividend_yield,
        #[java(set_as = crate::types::enum_types::DerivativeTypes)]
        stock_derivatives,
        board,
    ]
);

impl_java_class!(
    "com/longport/quote/PrePostQuote",
    longport::quote::PrePostQuote,
    [
        last_done, timestamp, volume, turnover, high, low, prev_close
    ]
);

impl_java_class!(
    "com/longport/quote/SecurityQuote",
    longport::quote::SecurityQuote,
    [
        symbol,
        last_done,
        prev_close,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status,
        pre_market_quote,
        post_market_quote,
        overnight_quote
    ]
);

impl_java_class!(
    "com/longport/quote/OptionQuote",
    longport::quote::OptionQuote,
    [
        symbol,
        last_done,
        prev_close,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status,
        implied_volatility,
        open_interest,
        expiry_date,
        strike_price,
        contract_multiplier,
        contract_type,
        contract_size,
        direction,
        historical_volatility,
        underlying_symbol,
    ]
);

impl_java_class!(
    "com/longport/quote/WarrantQuote",
    longport::quote::WarrantQuote,
    [
        symbol,
        last_done,
        prev_close,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status,
        implied_volatility,
        expiry_date,
        last_trade_date,
        outstanding_ratio,
        outstanding_quantity,
        conversion_ratio,
        category,
        strike_price,
        upper_strike_price,
        lower_strike_price,
        call_price,
        underlying_symbol
    ]
);

impl_java_class!(
    "com/longport/quote/SecurityDepth",
    longport::quote::SecurityDepth,
    [
        #[java(objarray)]
        asks,
        #[java(objarray)]
        bids
    ]
);

impl_java_class!(
    "com/longport/quote/SecurityBrokers",
    longport::quote::SecurityBrokers,
    [
        #[java(objarray)]
        ask_brokers,
        #[java(objarray)]
        bid_brokers
    ]
);

impl_java_class!(
    "com/longport/quote/ParticipantInfo",
    longport::quote::ParticipantInfo,
    [
        #[java(priarray)]
        broker_ids,
        name_cn,
        name_en,
        name_hk
    ]
);

impl_java_class!(
    "com/longport/quote/IntradayLine",
    longport::quote::IntradayLine,
    [price, timestamp, volume, turnover, avg_price]
);

impl_java_class!(
    "com/longport/quote/Candlestick",
    longport::quote::Candlestick,
    [
        close,
        open,
        low,
        high,
        volume,
        turnover,
        timestamp,
        trade_session
    ],
    non_exhaustive
);

impl_java_class!(
    "com/longport/quote/StrikePriceInfo",
    longport::quote::StrikePriceInfo,
    [price, call_symbol, put_symbol, standard]
);

impl_java_class!(
    "com/longport/quote/IssuerInfo",
    longport::quote::IssuerInfo,
    [issuer_id, name_cn, name_en, name_hk]
);

impl_java_class!(
    "com/longport/quote/MarketTradingSession",
    longport::quote::MarketTradingSession,
    [
        market,
        #[java(objarray)]
        trade_sessions
    ]
);

impl_java_class!(
    "com/longport/quote/TradingSessionInfo",
    longport::quote::TradingSessionInfo,
    [begin_time, end_time, trade_session]
);

impl_java_class!(
    "com/longport/quote/MarketTradingDays",
    longport::quote::MarketTradingDays,
    [
        #[java(objarray)]
        trading_days,
        #[java(objarray)]
        half_trading_days
    ]
);

impl_java_class!(
    "com/longport/quote/CapitalFlowLine",
    longport::quote::CapitalFlowLine,
    [inflow, timestamp]
);

impl_java_class!(
    "com/longport/quote/CapitalDistribution",
    longport::quote::CapitalDistribution,
    [large, medium, small]
);

pub(crate) struct SecurityCalcIndex {
    pub(crate) symbol: String,
    pub(crate) last_done: Option<Decimal>,
    pub(crate) change_value: Option<Decimal>,
    pub(crate) change_rate: Option<Decimal>,
    pub(crate) volume: i64,
    pub(crate) turnover: Option<Decimal>,
    pub(crate) ytd_change_rate: Option<Decimal>,
    pub(crate) turnover_rate: Option<Decimal>,
    pub(crate) total_market_value: Option<Decimal>,
    pub(crate) capital_flow: Option<Decimal>,
    pub(crate) amplitude: Option<Decimal>,
    pub(crate) volume_ratio: Option<Decimal>,
    pub(crate) pe_ttm_ratio: Option<Decimal>,
    pub(crate) pb_ratio: Option<Decimal>,
    pub(crate) dividend_ratio_ttm: Option<Decimal>,
    pub(crate) five_day_change_rate: Option<Decimal>,
    pub(crate) ten_day_change_rate: Option<Decimal>,
    pub(crate) half_year_change_rate: Option<Decimal>,
    pub(crate) five_minutes_change_rate: Option<Decimal>,
    pub(crate) expiry_date: Option<Date>,
    pub(crate) strike_price: Option<Decimal>,
    pub(crate) upper_strike_price: Option<Decimal>,
    pub(crate) lower_strike_price: Option<Decimal>,
    pub(crate) outstanding_qty: i64,
    pub(crate) outstanding_ratio: Option<Decimal>,
    pub(crate) premium: Option<Decimal>,
    pub(crate) itm_otm: Option<Decimal>,
    pub(crate) implied_volatility: Option<Decimal>,
    pub(crate) warrant_delta: Option<Decimal>,
    pub(crate) call_price: Option<Decimal>,
    pub(crate) to_call_price: Option<Decimal>,
    pub(crate) effective_leverage: Option<Decimal>,
    pub(crate) leverage_ratio: Option<Decimal>,
    pub(crate) conversion_ratio: Option<Decimal>,
    pub(crate) balance_point: Option<Decimal>,
    pub(crate) open_interest: i64,
    pub(crate) delta: Option<Decimal>,
    pub(crate) gamma: Option<Decimal>,
    pub(crate) theta: Option<Decimal>,
    pub(crate) vega: Option<Decimal>,
    pub(crate) rho: Option<Decimal>,
}

impl From<longport::quote::SecurityCalcIndex> for SecurityCalcIndex {
    fn from(
        longport::quote::SecurityCalcIndex {
            symbol,
            last_done,
            change_value,
            change_rate,
            volume,
            turnover,
            ytd_change_rate,
            turnover_rate,
            total_market_value,
            capital_flow,
            amplitude,
            volume_ratio,
            pe_ttm_ratio,
            pb_ratio,
            dividend_ratio_ttm,
            five_day_change_rate,
            ten_day_change_rate,
            half_year_change_rate,
            five_minutes_change_rate,
            expiry_date,
            strike_price,
            upper_strike_price,
            lower_strike_price,
            outstanding_qty,
            outstanding_ratio,
            premium,
            itm_otm,
            implied_volatility,
            warrant_delta,
            call_price,
            to_call_price,
            effective_leverage,
            leverage_ratio,
            conversion_ratio,
            balance_point,
            open_interest,
            delta,
            gamma,
            theta,
            vega,
            rho,
        }: longport::quote::SecurityCalcIndex,
    ) -> Self {
        Self {
            symbol,
            last_done,
            change_value,
            change_rate,
            volume: volume.unwrap_or_default(),
            turnover,
            ytd_change_rate,
            turnover_rate,
            total_market_value,
            capital_flow,
            amplitude,
            volume_ratio,
            pe_ttm_ratio,
            pb_ratio,
            dividend_ratio_ttm,
            five_day_change_rate,
            ten_day_change_rate,
            half_year_change_rate,
            five_minutes_change_rate,
            expiry_date,
            strike_price,
            upper_strike_price,
            lower_strike_price,
            outstanding_qty: outstanding_qty.unwrap_or_default(),
            outstanding_ratio,
            premium,
            itm_otm,
            implied_volatility,
            warrant_delta,
            call_price,
            to_call_price,
            effective_leverage,
            leverage_ratio,
            conversion_ratio,
            balance_point,
            open_interest: open_interest.unwrap_or_default(),
            delta,
            gamma,
            theta,
            vega,
            rho,
        }
    }
}

impl_java_class!(
    "com/longport/quote/SecurityCalcIndex",
    SecurityCalcIndex,
    [
        symbol,
        last_done,
        change_value,
        change_rate,
        volume,
        turnover,
        ytd_change_rate,
        turnover_rate,
        total_market_value,
        capital_flow,
        amplitude,
        volume_ratio,
        pe_ttm_ratio,
        pb_ratio,
        dividend_ratio_ttm,
        five_day_change_rate,
        ten_day_change_rate,
        half_year_change_rate,
        five_minutes_change_rate,
        expiry_date,
        strike_price,
        upper_strike_price,
        lower_strike_price,
        outstanding_qty,
        outstanding_ratio,
        premium,
        itm_otm,
        implied_volatility,
        warrant_delta,
        call_price,
        to_call_price,
        effective_leverage,
        leverage_ratio,
        conversion_ratio,
        balance_point,
        open_interest,
        delta,
        gamma,
        theta,
        vega,
        rho
    ]
);

impl_java_class!(
    "com/longport/quote/WatchlistGroup",
    longport::quote::WatchlistGroup,
    [
        id,
        name,
        #[java(objarray)]
        securities
    ]
);

impl_java_class!(
    "com/longport/quote/WatchlistSecurity",
    longport::quote::WatchlistSecurity,
    [symbol, market, name, watched_price, watched_at, is_pinned]
);

pub(crate) struct CreateWatchlistGroupResponse {
    pub(crate) id: i64,
}

impl_java_class!(
    "com/longport/quote/CreateWatchlistGroupResponse",
    CreateWatchlistGroupResponse,
    [id]
);

impl_java_class!(
    "com/longport/quote/CapitalDistributionResponse",
    longport::quote::CapitalDistributionResponse,
    [timestamp, capital_in, capital_out]
);

impl_java_class!(
    "com/longport/quote/RealtimeQuote",
    longport::quote::RealtimeQuote,
    [
        symbol,
        last_done,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status
    ]
);

impl_java_class!(
    "com/longport/quote/WarrantInfo",
    longport::quote::WarrantInfo,
    [
        symbol,
        warrant_type,
        name,
        last_done,
        change_rate,
        change_value,
        volume,
        turnover,
        expiry_date,
        strike_price,
        upper_strike_price,
        lower_strike_price,
        outstanding_qty,
        outstanding_ratio,
        premium,
        itm_otm,
        implied_volatility,
        delta,
        call_price,
        to_call_price,
        effective_leverage,
        leverage_ratio,
        conversion_ratio,
        balance_point,
        status,
    ]
);

impl_java_class!(
    "com/longport/trade/AttachedOrderDetail",
    longport::trade::AttachedOrderDetail,
    [
        order_id,
        attached_type_display,
        trigger_price,
        quantity,
        executed_qty,
        status,
        updated_at,
        withdrawn,
        gtd,
        time_in_force,
        counter_id,
        trigger_status,
        executed_amount,
        tag,
        submitted_at,
        executed_price,
        force_only_rth,
        reviewed,
        activate_order_type,
        activate_rth,
        submit_price
    ]
);

impl_java_class!(
    "com/longport/trade/PushOrderChanged",
    longport::trade::PushOrderChanged,
    [
        side,
        stock_name,
        submitted_quantity,
        symbol,
        order_type,
        submitted_price,
        executed_quantity,
        executed_price,
        order_id,
        currency,
        status,
        submitted_at,
        updated_at,
        trigger_price,
        msg,
        tag,
        trigger_status,
        trigger_at,
        trailing_amount,
        trailing_percent,
        limit_offset,
        account_no,
        last_share,
        last_price,
        remark
    ]
);

impl_java_class!(
    "com/longport/trade/Execution",
    longport::trade::Execution,
    [order_id, trade_id, symbol, trade_done_at, quantity, price]
);

impl_java_class!(
    "com/longport/trade/AllExecutionsResponse",
    longport::trade::AllExecutionsResponse,
    [
        has_more,
        #[java(objarray)]
        trades
    ]
);

impl_java_class!(
    "com/longport/trade/Order",
    longport::trade::Order,
    [
        order_id,
        status,
        stock_name,
        quantity,
        executed_quantity,
        price,
        executed_price,
        submitted_at,
        side,
        symbol,
        order_type,
        last_done,
        trigger_price,
        msg,
        tag,
        time_in_force,
        expire_date,
        updated_at,
        trigger_at,
        trailing_amount,
        trailing_percent,
        limit_offset,
        trigger_status,
        currency,
        outside_rth,
        #[java(set_as_opt = crate::types::JavaInteger)]
        limit_depth_level,
        #[java(set_as_opt = crate::types::JavaInteger)]
        trigger_count,
        monitor_price,
        remark,
        #[java(objarray)]
        attached_orders
    ]
);

impl_java_class!(
    "com/longport/trade/SubmitOrderResponse",
    longport::trade::SubmitOrderResponse,
    [order_id]
);

impl_java_class!(
    "com/longport/trade/CashInfo",
    longport::trade::CashInfo,
    [
        withdraw_cash,
        available_cash,
        frozen_cash,
        settling_cash,
        currency
    ]
);

impl_java_class!(
    "com/longport/trade/FrozenTransactionFee",
    longport::trade::FrozenTransactionFee,
    [currency, frozen_transaction_fee]
);

impl_java_class!(
    "com/longport/trade/AccountBalance",
    longport::trade::AccountBalance,
    [
        total_cash,
        max_finance_amount,
        remaining_finance_amount,
        risk_level,
        margin_call,
        currency,
        #[java(objarray)]
        cash_infos,
        net_assets,
        init_margin,
        maintenance_margin,
        buy_power,
        #[java(objarray)]
        frozen_transaction_fees
    ]
);

impl_java_class!(
    "com/longport/trade/CashFlow",
    longport::trade::CashFlow,
    [
        transaction_flow_name,
        direction,
        business_type,
        balance,
        currency,
        business_time,
        symbol,
        description,
    ]
);

impl_java_class!(
    "com/longport/trade/FundPositionsResponse",
    longport::trade::FundPositionsResponse,
    [
        #[java(objarray)]
        channels
    ]
);

impl_java_class!(
    "com/longport/trade/FundPositionChannel",
    longport::trade::FundPositionChannel,
    [
        account_channel,
        #[java(objarray)]
        positions
    ]
);

impl_java_class!(
    "com/longport/trade/FundPosition",
    longport::trade::FundPosition,
    [
        symbol,
        current_net_asset_value,
        net_asset_value_day,
        symbol_name,
        currency,
        cost_net_asset_value,
        holding_units
    ]
);

pub(crate) struct StockPositionsResponse {
    channels: Vec<StockPositionChannel>,
}

impl From<longport::trade::StockPositionsResponse> for StockPositionsResponse {
    fn from(value: longport::trade::StockPositionsResponse) -> Self {
        Self {
            channels: value
                .channels
                .into_iter()
                .map(StockPositionChannel::from)
                .collect(),
        }
    }
}

impl_java_class!(
    "com/longport/trade/StockPositionsResponse",
    StockPositionsResponse,
    [
        #[java(objarray)]
        channels
    ]
);

pub(crate) struct StockPositionChannel {
    account_channel: String,
    positions: Vec<StockPosition>,
}

impl From<longport::trade::StockPositionChannel> for StockPositionChannel {
    fn from(value: longport::trade::StockPositionChannel) -> Self {
        Self {
            account_channel: value.account_channel,
            positions: value
                .positions
                .into_iter()
                .map(StockPosition::from)
                .collect(),
        }
    }
}

impl_java_class!(
    "com/longport/trade/StockPositionChannel",
    StockPositionChannel,
    [
        account_channel,
        #[java(objarray)]
        positions
    ]
);

pub(crate) struct StockPosition {
    symbol: String,
    symbol_name: String,
    quantity: Decimal,
    available_quantity: Decimal,
    currency: String,
    cost_price: Decimal,
    market: Market,
    init_quantity: Decimal,
}

impl From<longport::trade::StockPosition> for StockPosition {
    fn from(value: longport::trade::StockPosition) -> Self {
        Self {
            symbol: value.symbol,
            symbol_name: value.symbol_name,
            quantity: value.quantity,
            available_quantity: value.available_quantity,
            currency: value.currency,
            cost_price: value.cost_price,
            market: value.market,
            init_quantity: value.init_quantity.unwrap_or_default(),
        }
    }
}

impl_java_class!(
    "com/longport/trade/StockPosition",
    StockPosition,
    [
        symbol,
        symbol_name,
        quantity,
        available_quantity,
        currency,
        cost_price,
        market,
        init_quantity
    ]
);

impl_java_class!(
    "com/longport/trade/MarginRatio",
    longport::trade::MarginRatio,
    [im_factor, mm_factor, fm_factor]
);

impl_java_class!(
    "com/longport/trade/OrderHistoryDetail",
    longport::trade::OrderHistoryDetail,
    [price, quantity, status, msg, time]
);

impl_java_class!(
    "com/longport/trade/OrderChargeFee",
    longport::trade::OrderChargeFee,
    [code, name, amount, currency]
);

impl_java_class!(
    "com/longport/trade/OrderChargeItem",
    longport::trade::OrderChargeItem,
    [
        code,
        name,
        #[java(objarray)]
        fees
    ]
);

impl_java_class!(
    "com/longport/trade/OrderChargeDetail",
    longport::trade::OrderChargeDetail,
    [
        total_amount,
        currency,
        #[java(objarray)]
        items
    ]
);

impl_java_class!(
    "com/longport/trade/OrderDetail",
    longport::trade::OrderDetail,
    [
        order_id,
        status,
        stock_name,
        quantity,
        executed_quantity,
        price,
        executed_price,
        submitted_at,
        side,
        symbol,
        order_type,
        last_done,
        trigger_price,
        msg,
        tag,
        time_in_force,
        expire_date,
        updated_at,
        trigger_at,
        trailing_amount,
        trailing_percent,
        limit_offset,
        trigger_status,
        currency,
        outside_rth,
        #[java(set_as_opt = crate::types::JavaInteger)]
        limit_depth_level,
        #[java(set_as_opt = crate::types::JavaInteger)]
        trigger_count,
        monitor_price,
        remark,
        free_status,
        free_amount,
        free_currency,
        deductions_status,
        deductions_amount,
        deductions_currency,
        platform_deducted_status,
        platform_deducted_amount,
        platform_deducted_currency,
        #[java(objarray)]
        history,
        charge_detail,
        #[java(objarray)]
        attached_orders
    ]
);

impl_java_class!(
    "com/longport/trade/EstimateMaxPurchaseQuantityResponse",
    longport::trade::EstimateMaxPurchaseQuantityResponse,
    [cash_max_qty, margin_max_qty]
);

impl_java_class!(
    "com/longport/quote/QuotePackageDetail",
    longport::quote::QuotePackageDetail,
    [key, name, description, start_at, end_at]
);

impl_java_class!(
    "com/longport/quote/MarketTemperature",
    longport::quote::MarketTemperature,
    [temperature, description, valuation, sentiment, timestamp]
);

impl_java_class!(
    "com/longport/quote/HistoryMarketTemperatureResponse",
    longport::quote::HistoryMarketTemperatureResponse,
    [
        granularity,
        #[java(objarray)]
        records
    ]
);

impl_java_class!(
    "com/longport/quote/FilingItem",
    longport::quote::FilingItem,
    [
        id,
        title,
        description,
        file_name,
        #[java(objarray)]
        file_urls,
        published_at
    ]
);

impl_java_class!(
    "com/longport/content/TopicItem",
    longport::content::TopicItem,
    [
        id,
        title,
        description,
        url,
        published_at,
        comments_count,
        likes_count,
        shares_count
    ]
);

impl_java_class!(
    "com/longport/content/NewsItem",
    longport::content::NewsItem,
    [
        id,
        title,
        description,
        url,
        published_at,
        comments_count,
        likes_count,
        shares_count
    ]
);

impl_java_class!(
    "com/longport/content/TopicAuthor",
    longport::content::TopicAuthor,
    [member_id, name, avatar]
);

impl_java_class!(
    "com/longport/content/TopicImage",
    longport::content::TopicImage,
    [url, sm, lg]
);

impl_java_class!(
    "com/longport/content/OwnedTopic",
    longport::content::OwnedTopic,
    [
        id,
        title,
        description,
        body,
        author,
        #[java(objarray)]
        tickers,
        #[java(objarray)]
        hashtags,
        #[java(objarray)]
        images,
        likes_count,
        comments_count,
        views_count,
        shares_count,
        topic_type,
        detail_url,
        created_at,
        updated_at
    ]
);

// ── MarketContext types ───────────────────────────────────────────

impl_java_class!(
    "com/longport/market/MarketStatusResponse",
    longport::market::MarketStatusResponse,
    [
        #[java(objarray)]
        market_time
    ]
);

impl_java_class!(
    "com/longport/market/MarketTimeItem",
    longport::market::MarketTimeItem,
    [
        market,
        trade_status,
        timestamp,
        delay_trade_status,
        delay_timestamp,
        sub_status,
        delay_sub_status
    ]
);

impl_java_class!(
    "com/longport/market/BrokerHoldingTop",
    longport::market::BrokerHoldingTop,
    [
        #[java(objarray)]
        buy,
        #[java(objarray)]
        sell,
        updated_at
    ]
);

impl_java_class!(
    "com/longport/market/BrokerHoldingEntry",
    longport::market::BrokerHoldingEntry,
    [name, parti_number, chg, strong]
);

impl_java_class!(
    "com/longport/market/BrokerHoldingDetail",
    longport::market::BrokerHoldingDetail,
    [
        #[java(objarray)]
        list,
        updated_at
    ]
);

impl_java_class!(
    "com/longport/market/BrokerHoldingDetailItem",
    longport::market::BrokerHoldingDetailItem,
    [name, parti_number, ratio, shares, strong]
);

impl_java_class!(
    "com/longport/market/BrokerHoldingChanges",
    longport::market::BrokerHoldingChanges,
    [value, chg_1, chg_5, chg_20, chg_60]
);

impl_java_class!(
    "com/longport/market/BrokerHoldingDailyHistory",
    longport::market::BrokerHoldingDailyHistory,
    [
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longport/market/BrokerHoldingDailyItem",
    longport::market::BrokerHoldingDailyItem,
    [date, holding, ratio, chg]
);

impl_java_class!(
    "com/longport/market/AhPremiumKlines",
    longport::market::AhPremiumKlines,
    [
        #[java(objarray)]
        klines
    ]
);

impl_java_class!(
    "com/longport/market/AhPremiumIntraday",
    longport::market::AhPremiumIntraday,
    [
        #[java(objarray)]
        klines
    ]
);

impl_java_class!(
    "com/longport/market/AhPremiumKline",
    longport::market::AhPremiumKline,
    [
        aprice,
        apreclose,
        hprice,
        hpreclose,
        currency_rate,
        ahpremium_rate,
        price_spread,
        timestamp
    ]
);

impl_java_class!(
    "com/longport/market/TradeStatsResponse",
    longport::market::TradeStatsResponse,
    [
        statistics,
        #[java(objarray)]
        trades
    ]
);

impl_java_class!(
    "com/longport/market/TradeStatistics",
    longport::market::TradeStatistics,
    [
        avgprice,
        buy,
        neutral,
        preclose,
        sell,
        timestamp,
        total_amount,
        #[java(objarray)]
        trade_date,
        trades_count
    ]
);

impl_java_class!(
    "com/longport/market/TradePriceLevel",
    longport::market::TradePriceLevel,
    [buy_amount, neutral_amount, price, sell_amount]
);

impl_java_class!(
    "com/longport/market/AnomalyResponse",
    longport::market::AnomalyResponse,
    [
        all_off,
        #[java(objarray)]
        changes
    ]
);

impl_java_class!(
    "com/longport/market/AnomalyItem",
    longport::market::AnomalyItem,
    [
        symbol,
        name,
        alert_name,
        alert_time,
        #[java(objarray)]
        change_values,
        emotion
    ]
);

impl_java_class!(
    "com/longport/market/IndexConstituents",
    longport::market::IndexConstituents,
    [
        fall_num,
        flat_num,
        rise_num,
        #[java(objarray)]
        stocks
    ]
);

impl_java_class!(
    "com/longport/market/ConstituentStock",
    longport::market::ConstituentStock,
    [
        symbol,
        name,
        last_done,
        prev_close,
        inflow,
        balance,
        amount,
        total_shares,
        #[java(objarray)]
        tags,
        intro,
        market,
        circulating_shares,
        delay,
        chg,
        trade_status
    ]
);

// ── CalendarContext types ─────────────────────────────────────────

impl_java_class!(
    "com/longport/calendar/CalendarEventsResponse",
    longport::calendar::CalendarEventsResponse,
    [
        date,
        #[java(objarray)]
        list,
        next_date
    ]
);

impl_java_class!(
    "com/longport/calendar/CalendarDateGroup",
    longport::calendar::CalendarDateGroup,
    [
        date,
        count,
        #[java(objarray)]
        infos
    ]
);

impl_java_class!(
    "com/longport/calendar/CalendarEventInfo",
    longport::calendar::CalendarEventInfo,
    [
        symbol,
        market,
        content,
        counter_name,
        date_type,
        date,
        chart_uid,
        #[java(objarray)]
        data_kv,
        event_type,
        datetime,
        icon,
        star,
        live,
        id,
        financial_market_time,
        currency,
        ext,
        activity_type
    ]
);

impl_java_class!(
    "com/longport/calendar/CalendarDataKv",
    longport::calendar::CalendarDataKv,
    [key, value, value_type, value_raw]
);

// ── PortfolioContext types ────────────────────────────────────────

impl_java_class!(
    "com/longport/portfolio/ExchangeRates",
    longport::portfolio::ExchangeRates,
    [
        #[java(objarray)]
        exchanges
    ]
);

impl_java_class!(
    "com/longport/portfolio/ExchangeRate",
    longport::portfolio::ExchangeRate,
    [
        average_rate,
        base_currency,
        bid_rate,
        offer_rate,
        other_currency
    ]
);

impl_java_class!(
    "com/longport/portfolio/ProfitAnalysis",
    longport::portfolio::ProfitAnalysis,
    [summary, sublist]
);

impl_java_class!(
    "com/longport/portfolio/ProfitAnalysisSummary",
    longport::portfolio::ProfitAnalysisSummary,
    [
        currency,
        current_total_asset,
        start_date,
        end_date,
        start_time,
        end_time,
        ending_asset_value,
        initial_asset_value,
        invest_amount,
        is_traded,
        sum_profit,
        sum_profit_rate,
        profits
    ]
);

impl_java_class!(
    "com/longport/portfolio/ProfitSummaryBreakdown",
    longport::portfolio::ProfitSummaryBreakdown,
    [
        stock,
        fund,
        crypto,
        mmf,
        other,
        cumulative_transaction_amount,
        trade_order_num,
        trade_stock_num,
        ipo,
        ipo_hit,
        ipo_subscription,
        #[java(objarray)]
        summary_info
    ]
);

impl_java_class!(
    "com/longport/portfolio/ProfitSummaryInfo",
    longport::portfolio::ProfitSummaryInfo,
    [
        asset_type,
        profit_max,
        profit_max_name,
        loss_max,
        loss_max_name
    ]
);

impl_java_class!(
    "com/longport/portfolio/ProfitAnalysisSublist",
    longport::portfolio::ProfitAnalysisSublist,
    [
        start,
        end,
        start_date,
        end_date,
        updated_at,
        updated_date,
        #[java(objarray)]
        items
    ]
);

impl_java_class!(
    "com/longport/portfolio/ProfitAnalysisItem",
    longport::portfolio::ProfitAnalysisItem,
    [
        name,
        market,
        is_holding,
        profit,
        profit_rate,
        clearance_times,
        item_type,
        currency,
        symbol,
        holding_period,
        security_code,
        isin,
        underlying_profit,
        derivatives_profit,
        order_profit
    ]
);

impl_java_class!(
    "com/longport/portfolio/ProfitAnalysisDetail",
    longport::portfolio::ProfitAnalysisDetail,
    [
        profit,
        underlying_details,
        derivative_pnl_details,
        name,
        updated_at,
        updated_date,
        currency,
        default_tag,
        start,
        end,
        start_date,
        end_date
    ]
);

impl_java_class!(
    "com/longport/portfolio/ProfitDetails",
    longport::portfolio::ProfitDetails,
    [
        holding_value,
        profit,
        cumulative_credited_amount,
        #[java(objarray)]
        credited_details,
        cumulative_debited_amount,
        #[java(objarray)]
        debited_details,
        cumulative_fee_amount,
        #[java(objarray)]
        fee_details,
        short_holding_value,
        long_holding_value,
        holding_value_at_beginning,
        holding_value_at_ending
    ]
);

impl_java_class!(
    "com/longport/portfolio/ProfitDetailEntry",
    longport::portfolio::ProfitDetailEntry,
    [describe, amount]
);

impl_java_class!(
    "com/longport/portfolio/ProfitAnalysisByMarketItem",
    longport::portfolio::ProfitAnalysisByMarketItem,
    [code, name, market, profit]
);

impl_java_class!(
    "com/longport/portfolio/ProfitAnalysisByMarket",
    longport::portfolio::ProfitAnalysisByMarket,
    [
        profit,
        has_more,
        #[java(objarray)]
        stock_items
    ]
);

// ── DcaPlan and friends ───────────────────────────────────────────

impl_java_class!(
    "com/longport/dca/DcaPlan",
    longport::dca::DcaPlan,
    [
        plan_id,
        status,
        symbol,
        member_id,
        aaid,
        account_channel,
        display_account,
        market,
        per_invest_amount,
        invest_frequency,
        invest_day_of_week,
        invest_day_of_month,
        allow_margin_finance,
        alter_hours,
        created_at,
        updated_at,
        next_trd_date,
        stock_name,
        cum_amount,
        issue_number,
        average_cost,
        cum_profit
    ]
);

impl_java_class!(
    "com/longport/dca/DcaList",
    longport::dca::DcaList,
    [
        #[java(objarray)]
        plans
    ]
);

impl_java_class!(
    "com/longport/dca/DcaStats",
    longport::dca::DcaStats,
    [
        active_count,
        finished_count,
        suspended_count,
        #[java(objarray)]
        nearest_plans,
        rest_days,
        total_amount,
        total_profit
    ]
);

impl_java_class!(
    "com/longport/dca/DcaCreateResult",
    longport::dca::DcaCreateResult,
    [plan_id]
);

// ── SharelistContext types ────────────────────────────────────────

impl_java_class!(
    "com/longport/sharelist/SharelistStock",
    longport::sharelist::SharelistStock,
    [
        symbol,
        name,
        market,
        code,
        intro,
        unread_change_log_category,
        change,
        last_done,
        trade_status,
        latency
    ]
);

impl_java_class!(
    "com/longport/sharelist/SharelistScopes",
    longport::sharelist::SharelistScopes,
    [subscription, is_self]
);

impl_java_class!(
    "com/longport/sharelist/SharelistInfo",
    longport::sharelist::SharelistInfo,
    [
        id,
        name,
        description,
        cover,
        subscribers_count,
        created_at,
        edited_at,
        this_year_chg,
        creator,
        #[java(objarray)]
        stocks,
        subscribed,
        chg,
        sharelist_type,
        industry_code
    ]
);

impl_java_class!(
    "com/longport/sharelist/SharelistList",
    longport::sharelist::SharelistList,
    [
        #[java(objarray)]
        sharelists,
        #[java(objarray)]
        subscribed_sharelists,
        tail_mark
    ]
);

impl_java_class!(
    "com/longport/sharelist/SharelistDetail",
    longport::sharelist::SharelistDetail,
    [sharelist, scopes]
);
// ── DCAContext types ──────────────────────────────────────────────

impl_java_class!(
    "com/longport/dca/DcaHistoryRecord",
    longport::dca::DcaHistoryRecord,
    [
        created_at,
        order_id,
        status,
        action,
        order_type,
        executed_qty,
        executed_price,
        executed_amount,
        rejected_reason,
        symbol
    ]
);

impl_java_class!(
    "com/longport/dca/DcaHistoryResponse",
    longport::dca::DcaHistoryResponse,
    [
        #[java(objarray)]
        records,
        has_more
    ]
);

impl_java_class!(
    "com/longport/dca/DcaSupportInfo",
    longport::dca::DcaSupportInfo,
    [symbol, support_regular_saving]
);

impl_java_class!(
    "com/longport/dca/DcaSupportList",
    longport::dca::DcaSupportList,
    [
        #[java(objarray)]
        infos
    ]
);

impl_java_class!(
    "com/longport/dca/DcaCalcDateResult",
    longport::dca::DcaCalcDateResult,
    [trade_date]
);

// DcaPlan has serde_json::Value creator field - use JSON for DcaList
// ── AlertContext types ────────────────────────────────────────────

impl_java_class!(
    "com/longport/alert/AlertValueMap",
    longport::alert::AlertValueMap,
    [
        price,
        #[java(set_as_opt = crate::types::JavaDouble)]
        chg
    ]
);

impl_java_class!(
    "com/longport/alert/AlertItem",
    longport::alert::AlertItem,
    [
        id,
        indicator_id,
        enabled,
        frequency,
        scope,
        text,
        #[java(priarray)]
        state,
        value_map
    ]
);

impl_java_class!(
    "com/longport/alert/AlertSymbolGroup",
    longport::alert::AlertSymbolGroup,
    [
        symbol,
        code,
        market,
        name,
        price,
        chg,
        p_chg,
        product,
        #[java(objarray)]
        indicators
    ]
);

impl_java_class!(
    "com/longport/alert/AlertList",
    longport::alert::AlertList,
    [
        #[java(objarray)]
        lists
    ]
);
// ── FundamentalContext types ──────────────────────────────────────

impl_java_class!(
    "com/longport/fundamental/FinancialReports",
    longport::fundamental::FinancialReports,
    [list]
);

impl_java_class!(
    "com/longport/fundamental/DividendList",
    longport::fundamental::DividendList,
    [
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longport/fundamental/DividendItem",
    longport::fundamental::DividendItem,
    [symbol, id, desc, record_date, ex_date, payment_date]
);

impl_java_class!(
    "com/longport/fundamental/InstitutionRating",
    longport::fundamental::InstitutionRating,
    [latest, summary]
);

impl_java_class!(
    "com/longport/fundamental/InstitutionRatingLatest",
    longport::fundamental::InstitutionRatingLatest,
    [
        evaluate,
        target,
        industry_id,
        industry_name,
        industry_rank,
        industry_total,
        industry_mean,
        industry_median
    ]
);

impl_java_class!(
    "com/longport/fundamental/RatingEvaluate",
    longport::fundamental::RatingEvaluate,
    [
        buy, over, hold, under, sell, no_opinion, total, start_date, end_date
    ]
);

impl_java_class!(
    "com/longport/fundamental/RatingTarget",
    longport::fundamental::RatingTarget,
    [
        highest_price,
        lowest_price,
        prev_close,
        start_date,
        end_date
    ]
);

impl_java_class!(
    "com/longport/fundamental/InstitutionRatingSummary",
    longport::fundamental::InstitutionRatingSummary,
    [ccy_symbol, change, evaluate, recommend, target, updated_at]
);

impl_java_class!(
    "com/longport/fundamental/RatingSummaryEvaluate",
    longport::fundamental::RatingSummaryEvaluate,
    [buy, date, hold, sell, strong_buy, under]
);

impl_java_class!(
    "com/longport/fundamental/InstitutionRatingDetail",
    longport::fundamental::InstitutionRatingDetail,
    [ccy_symbol, evaluate, target]
);

impl_java_class!(
    "com/longport/fundamental/InstitutionRatingDetailEvaluate",
    longport::fundamental::InstitutionRatingDetailEvaluate,
    [
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longport/fundamental/InstitutionRatingDetailEvaluateItem",
    longport::fundamental::InstitutionRatingDetailEvaluateItem,
    [buy, date, hold, sell, strong_buy, no_opinion, under]
);

impl_java_class!(
    "com/longport/fundamental/InstitutionRatingDetailTarget",
    longport::fundamental::InstitutionRatingDetailTarget,
    [
        data_percent,
        prediction_accuracy,
        updated_at,
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longport/fundamental/InstitutionRatingDetailTargetItem",
    longport::fundamental::InstitutionRatingDetailTargetItem,
    [
        avg_target, date, max_target, min_target, meet, price, timestamp
    ]
);

impl_java_class!(
    "com/longport/fundamental/ForecastEps",
    longport::fundamental::ForecastEps,
    [
        #[java(objarray)]
        items
    ]
);

impl_java_class!(
    "com/longport/fundamental/ForecastEpsItem",
    longport::fundamental::ForecastEpsItem,
    [
        forecast_eps_median,
        forecast_eps_mean,
        forecast_eps_lowest,
        forecast_eps_highest,
        institution_total,
        institution_up,
        institution_down,
        forecast_start_date,
        forecast_end_date
    ]
);

impl_java_class!(
    "com/longport/fundamental/FinancialConsensus",
    longport::fundamental::FinancialConsensus,
    [
        #[java(objarray)]
        list,
        current_index,
        currency,
        #[java(objarray)]
        opt_periods,
        current_period
    ]
);

impl_java_class!(
    "com/longport/fundamental/ConsensusReport",
    longport::fundamental::ConsensusReport,
    [
        fiscal_year,
        fiscal_period,
        period_text,
        #[java(objarray)]
        details
    ]
);

impl_java_class!(
    "com/longport/fundamental/ConsensusDetail",
    longport::fundamental::ConsensusDetail,
    [
        key,
        name,
        description,
        actual,
        estimate,
        comp_value,
        comp_desc,
        comp,
        is_released
    ]
);

impl_java_class!(
    "com/longport/fundamental/ValuationData",
    longport::fundamental::ValuationData,
    [metrics]
);

impl_java_class!(
    "com/longport/fundamental/ValuationMetricsData",
    longport::fundamental::ValuationMetricsData,
    [pe, pb, ps, dvd_yld]
);

impl_java_class!(
    "com/longport/fundamental/ValuationMetricData",
    longport::fundamental::ValuationMetricData,
    [
        desc,
        high,
        low,
        median,
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longport/fundamental/ValuationPoint",
    longport::fundamental::ValuationPoint,
    [timestamp, value]
);

impl_java_class!(
    "com/longport/fundamental/ValuationHistoryResponse",
    longport::fundamental::ValuationHistoryResponse,
    [history]
);

impl_java_class!(
    "com/longport/fundamental/ValuationHistoryData",
    longport::fundamental::ValuationHistoryData,
    [metrics]
);

impl_java_class!(
    "com/longport/fundamental/ValuationHistoryMetrics",
    longport::fundamental::ValuationHistoryMetrics,
    [pe, pb, ps]
);

impl_java_class!(
    "com/longport/fundamental/ValuationHistoryMetric",
    longport::fundamental::ValuationHistoryMetric,
    [
        desc,
        high,
        low,
        median,
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longport/fundamental/IndustryValuationList",
    longport::fundamental::IndustryValuationList,
    [
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longport/fundamental/IndustryValuationItem",
    longport::fundamental::IndustryValuationItem,
    [
        symbol,
        name,
        currency,
        assets,
        bps,
        eps,
        dps,
        div_yld,
        div_payout_ratio,
        five_y_avg_dps,
        pe,
        #[java(objarray)]
        history
    ]
);

impl_java_class!(
    "com/longport/fundamental/IndustryValuationHistory",
    longport::fundamental::IndustryValuationHistory,
    [date, pe, pb, ps]
);

impl_java_class!(
    "com/longport/fundamental/IndustryValuationDist",
    longport::fundamental::IndustryValuationDist,
    [pe, pb, ps]
);

impl_java_class!(
    "com/longport/fundamental/ValuationDist",
    longport::fundamental::ValuationDist,
    [low, high, median, value, ranking, rank_index, rank_total]
);

impl_java_class!(
    "com/longport/fundamental/CompanyOverview",
    longport::fundamental::CompanyOverview,
    [
        name,
        company_name,
        founded,
        listing_date,
        market,
        region,
        address,
        office_address,
        website,
        issue_price,
        shares_offered,
        chairman,
        secretary,
        audit_inst,
        category,
        year_end,
        employees,
        phone,
        fax,
        email,
        legal_repr,
        manager,
        bus_license,
        accounting_firm,
        securities_rep,
        legal_counsel,
        zip_code,
        ticker,
        icon,
        profile,
        ads_ratio,
        sector
    ]
);

impl_java_class!(
    "com/longport/fundamental/ExecutiveList",
    longport::fundamental::ExecutiveList,
    [
        #[java(objarray)]
        professional_list
    ]
);

impl_java_class!(
    "com/longport/fundamental/ExecutiveGroup",
    longport::fundamental::ExecutiveGroup,
    [
        symbol,
        forward_url,
        total,
        #[java(objarray)]
        professionals
    ]
);

impl_java_class!(
    "com/longport/fundamental/Professional",
    longport::fundamental::Professional,
    [
        id, name, name_zhcn, name_en, title, biography, photo, wiki_url
    ]
);

impl_java_class!(
    "com/longport/fundamental/ShareholderList",
    longport::fundamental::ShareholderList,
    [
        #[java(objarray)]
        shareholder_list,
        forward_url,
        total
    ]
);

impl_java_class!(
    "com/longport/fundamental/Shareholder",
    longport::fundamental::Shareholder,
    [
        shareholder_id,
        shareholder_name,
        institution_type,
        percent_of_shares,
        shares_changed,
        report_date,
        #[java(objarray)]
        stocks
    ]
);

impl_java_class!(
    "com/longport/fundamental/ShareholderStock",
    longport::fundamental::ShareholderStock,
    [symbol, code, market, chg]
);

impl_java_class!(
    "com/longport/fundamental/FundHolders",
    longport::fundamental::FundHolders,
    [
        #[java(objarray)]
        lists
    ]
);

impl_java_class!(
    "com/longport/fundamental/FundHolder",
    longport::fundamental::FundHolder,
    [code, symbol, currency, name, position_ratio, report_date]
);

impl_java_class!(
    "com/longport/fundamental/CorpActionLive",
    longport::fundamental::CorpActionLive,
    [id, status, started_at, name, icon]
);

impl_java_class!(
    "com/longport/fundamental/CorpActions",
    longport::fundamental::CorpActions,
    [
        #[java(objarray)]
        items
    ]
);

impl_java_class!(
    "com/longport/fundamental/CorpActionItem",
    longport::fundamental::CorpActionItem,
    [
        id,
        date,
        date_str,
        date_type,
        date_zone,
        act_type,
        act_desc,
        action,
        recent,
        is_delay,
        delay_content,
        live,
        security
    ]
);

impl_java_class!(
    "com/longport/fundamental/InvestRelations",
    longport::fundamental::InvestRelations,
    [
        forward_url,
        #[java(objarray)]
        invest_securities
    ]
);

impl_java_class!(
    "com/longport/fundamental/InvestSecurity",
    longport::fundamental::InvestSecurity,
    [
        company_id,
        company_name,
        company_name_en,
        company_name_zhcn,
        symbol,
        currency,
        percent_of_shares,
        shares_rank,
        shares_value
    ]
);

impl_java_class!(
    "com/longport/fundamental/OperatingList",
    longport::fundamental::OperatingList,
    [
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longport/fundamental/OperatingItem",
    longport::fundamental::OperatingItem,
    [
        id,
        report,
        title,
        txt,
        latest,
        web_url,
        financial,
        #[java(objarray)]
        keywords
    ]
);

impl_java_class!(
    "com/longport/fundamental/OperatingFinancial",
    longport::fundamental::OperatingFinancial,
    [
        code,
        symbol,
        currency,
        name,
        region,
        report,
        report_txt,
        #[java(objarray)]
        indicators
    ]
);

impl_java_class!(
    "com/longport/fundamental/OperatingIndicator",
    longport::fundamental::OperatingIndicator,
    [field_name, indicator_name, indicator_value, yoy]
);

// ── QuoteContext extensions ───────────────────────────────────────

impl_java_class!(
    "com/longport/quote/ShortPositionsItem",
    longport::quote::ShortPositionsItem,
    [
        timestamp,
        rate,
        close,
        current_shares_short,
        avg_daily_share_volume,
        days_to_cover,
        amount,
        balance,
        cost
    ]
);

impl_java_class!(
    "com/longport/quote/ShortPositionsResponse",
    longport::quote::ShortPositionsResponse,
    [
        #[java(objarray)]
        data
    ]
);

impl_java_class!(
    "com/longport/quote/ShortTradesItem",
    longport::quote::ShortTradesItem,
    [
        timestamp,
        rate,
        close,
        nus_amount,
        ny_amount,
        total_amount,
        amount,
        balance
    ]
);

impl_java_class!(
    "com/longport/quote/ShortTradesResponse",
    longport::quote::ShortTradesResponse,
    [
        #[java(objarray)]
        data
    ]
);

impl_java_class!(
    "com/longport/quote/OptionVolumeStats",
    longport::quote::OptionVolumeStats,
    [symbol, call_volume, put_volume]
);

impl_java_class!(
    "com/longport/quote/OptionVolumeDaily",
    longport::quote::OptionVolumeDaily,
    [
        symbol,
        #[java(objarray)]
        stats
    ]
);

impl_java_class!(
    "com/longport/quote/OptionVolumeDailyStat",
    longport::quote::OptionVolumeDailyStat,
    [
        symbol,
        date,
        call_volume,
        put_volume,
        call_open_interest,
        put_open_interest,
        total_volume,
        total_open_interest,
        pc_vol,
        pc_oi
    ]
);

// ── FundamentalContext: BuybackData and related ───────────────────

impl_java_class!(
    "com/longport/fundamental/RecentBuybacks",
    longport::fundamental::RecentBuybacks,
    [currency, net_buyback_ttm, net_buyback_yield_ttm]
);

impl_java_class!(
    "com/longport/fundamental/BuybackHistoryItem",
    longport::fundamental::BuybackHistoryItem,
    [
        fiscal_year,
        fiscal_year_range,
        net_buyback,
        net_buyback_yield,
        net_buyback_growth_rate,
        currency
    ]
);

impl_java_class!(
    "com/longport/fundamental/BuybackRatios",
    longport::fundamental::BuybackRatios,
    [net_buyback_payout_ratio, net_buyback_to_cashflow_ratio]
);

impl_java_class!(
    "com/longport/fundamental/BuybackData",
    longport::fundamental::BuybackData,
    [
        recent_buybacks,
        #[java(objarray)]
        buyback_history,
        #[java(objarray)]
        buyback_ratios
    ]
);

// ── FundamentalContext: StockRatings and related ──────────────────

impl_java_class!(
    "com/longport/fundamental/RatingIndicator",
    longport::fundamental::RatingIndicator,
    [
        name,
        #[java(set_as_opt = crate::types::JavaDouble)]
        score,
        letter
    ]
);

impl_java_class!(
    "com/longport/fundamental/RatingLeafIndicator",
    longport::fundamental::RatingLeafIndicator,
    [
        name,
        value,
        value_type,
        #[java(set_as_opt = crate::types::JavaDouble)]
        score,
        letter
    ]
);

impl_java_class!(
    "com/longport/fundamental/RatingSubIndicatorGroup",
    longport::fundamental::RatingSubIndicatorGroup,
    [
        indicator,
        #[java(objarray)]
        sub_indicators
    ]
);

impl_java_class!(
    "com/longport/fundamental/RatingCategory",
    longport::fundamental::RatingCategory,
    [
        kind,
        #[java(objarray)]
        sub_indicators
    ]
);

impl_java_class!(
    "com/longport/fundamental/StockRatings",
    longport::fundamental::StockRatings,
    [
        style_txt_name,
        scale_txt_name,
        report_period_txt,
        #[java(set_as_opt = crate::types::JavaDouble)]
        multi_score,
        multi_letter,
        multi_score_change,
        industry_name,
        #[java(set_as_opt = crate::types::JavaLong)]
        industry_rank,
        #[java(set_as_opt = crate::types::JavaLong)]
        industry_total,
        #[java(set_as_opt = crate::types::JavaDouble)]
        industry_mean_score,
        #[java(set_as_opt = crate::types::JavaDouble)]
        industry_median_score,
        #[java(objarray)]
        ratings
    ]
);

// ── FundamentalContext: new APIs ──────────────────────────────────

impl_java_class!(
    "com/longport/fundamental/BusinessSegmentItem",
    longport::fundamental::BusinessSegmentItem,
    [name, percent]
);

impl_java_class!(
    "com/longport/fundamental/BusinessSegments",
    longport::fundamental::BusinessSegments,
    [
        date,
        total,
        currency,
        #[java(objarray)]
        business
    ]
);

impl_java_class!(
    "com/longport/fundamental/BusinessSegmentHistoryItem",
    longport::fundamental::BusinessSegmentHistoryItem,
    [name, percent, value]
);

impl_java_class!(
    "com/longport/fundamental/BusinessSegmentsHistoricalItem",
    longport::fundamental::BusinessSegmentsHistoricalItem,
    [
        date,
        total,
        currency,
        #[java(objarray)]
        business,
        #[java(objarray)]
        regionals
    ]
);

impl_java_class!(
    "com/longport/fundamental/BusinessSegmentsHistory",
    longport::fundamental::BusinessSegmentsHistory,
    [
        #[java(objarray)]
        historical
    ]
);

impl_java_class!(
    "com/longport/fundamental/InstitutionRatingViewItem",
    longport::fundamental::InstitutionRatingViewItem,
    [date, buy, over, hold, under, sell, total]
);

impl_java_class!(
    "com/longport/fundamental/InstitutionRatingViews",
    longport::fundamental::InstitutionRatingViews,
    [
        #[java(objarray)]
        elist
    ]
);

impl_java_class!(
    "com/longport/fundamental/IndustryRankItem",
    longport::fundamental::IndustryRankItem,
    [
        name,
        symbol,
        chg,
        leading_name,
        leading_ticker,
        leading_chg,
        value_name,
        value_data
    ]
);

impl_java_class!(
    "com/longport/fundamental/IndustryRankGroup",
    longport::fundamental::IndustryRankGroup,
    [
        #[java(objarray)]
        lists
    ]
);

impl_java_class!(
    "com/longport/fundamental/IndustryRankResponse",
    longport::fundamental::IndustryRankResponse,
    [
        #[java(objarray)]
        items
    ]
);

impl_java_class!(
    "com/longport/fundamental/IndustryPeersTop",
    longport::fundamental::IndustryPeersTop,
    [name, market]
);

// IndustryPeerNode has a recursive `next` field; we serialize it as nextJson.
// Manual impl (macro can't rename fields).
#[allow(non_upper_case_globals)]
static com_longport_fundamental_IndustryPeerNode: std::sync::OnceLock<jni::objects::GlobalRef> =
    std::sync::OnceLock::new();

impl crate::types::ClassLoader for longport::fundamental::IndustryPeerNode {
    fn init(env: &mut jni::JNIEnv) {
        let cls = jni::descriptors::Desc::<jni::objects::JClass>::lookup(
            "com/longport/fundamental/IndustryPeerNode",
            env,
        )
        .expect("com/longport/fundamental/IndustryPeerNode");
        let _ = com_longport_fundamental_IndustryPeerNode.set(env.new_global_ref(&*cls).unwrap());
    }

    fn class_ref() -> jni::objects::GlobalRef {
        com_longport_fundamental_IndustryPeerNode
            .get()
            .cloned()
            .unwrap()
    }
}

impl crate::types::JSignature for longport::fundamental::IndustryPeerNode {
    #[inline]
    fn signature() -> ::std::borrow::Cow<'static, str> {
        "Lcom/longport/fundamental/IndustryPeerNode;".into()
    }
}

impl crate::types::IntoJValue for longport::fundamental::IndustryPeerNode {
    fn into_jvalue<'a>(
        self,
        env: &mut jni::JNIEnv<'a>,
    ) -> jni::errors::Result<jni::objects::JValueOwned<'a>> {
        let longport::fundamental::IndustryPeerNode {
            name,
            symbol,
            stock_num,
            chg,
            ytd_chg,
            next,
        } = self;
        let next_json = serde_json::to_string(&next).unwrap_or_default();
        let cls = <Self as crate::types::ClassLoader>::class_ref();
        let obj = env.new_object(cls.borrow(), "()V", &[])?;
        crate::types::set_field(env, &obj, "name", name)?;
        crate::types::set_field(env, &obj, "symbol", symbol)?;
        crate::types::set_field(env, &obj, "stockNum", stock_num)?;
        crate::types::set_field(env, &obj, "chg", chg)?;
        crate::types::set_field(env, &obj, "ytdChg", ytd_chg)?;
        crate::types::set_field(env, &obj, "nextJson", next_json)?;
        Ok(obj.into())
    }
}

impl_java_class!(
    "com/longport/fundamental/IndustryPeersResponse",
    longport::fundamental::IndustryPeersResponse,
    [top, chain]
);

impl_java_class!(
    "com/longport/fundamental/SnapshotForecastMetric",
    longport::fundamental::SnapshotForecastMetric,
    [value, yoy, cmp_desc, est_value]
);

impl_java_class!(
    "com/longport/fundamental/SnapshotReportedMetric",
    longport::fundamental::SnapshotReportedMetric,
    [value, yoy]
);

impl_java_class!(
    "com/longport/fundamental/FinancialReportSnapshot",
    longport::fundamental::FinancialReportSnapshot,
    [
        name,
        ticker,
        fp_start,
        fp_end,
        currency,
        report_desc,
        #[java(nullable)]
        fo_revenue,
        #[java(nullable)]
        fo_ebit,
        #[java(nullable)]
        fo_eps,
        #[java(nullable)]
        fr_revenue,
        #[java(nullable)]
        fr_profit,
        #[java(nullable)]
        fr_operate_cash,
        #[java(nullable)]
        fr_invest_cash,
        #[java(nullable)]
        fr_finance_cash,
        #[java(nullable)]
        fr_total_assets,
        #[java(nullable)]
        fr_total_liability,
        fr_roe_ttm,
        fr_profit_margin,
        fr_profit_margin_ttm,
        fr_asset_turn_ttm,
        fr_leverage_ttm,
        fr_debt_assets_ratio
    ]
);

// ── PortfolioContext: ProfitAnalysisFlows and related ─────────────

impl_java_class!(
    "com/longport/portfolio/FlowItem",
    longport::portfolio::FlowItem,
    [
        executed_date,
        #[java(nullable)]
        executed_timestamp,
        code,
        direction,
        executed_quantity,
        executed_price,
        executed_cost,
        describe
    ]
);

impl_java_class!(
    "com/longport/portfolio/ProfitAnalysisFlowsResponse",
    longport::portfolio::ProfitAnalysisFlows,
    [
        #[java(objarray)]
        flows_list,
        has_more
    ]
);

// ── FundamentalContext: shareholders / valuation comparison ────────

impl_java_class!(
    "com/longport/fundamental/ShareholderTopResponse",
    longport::fundamental::ShareholderTopResponse,
    [data]
);

impl_java_class!(
    "com/longport/fundamental/ShareholderDetailResponse",
    longport::fundamental::ShareholderDetailResponse,
    [data]
);

impl_java_class!(
    "com/longport/fundamental/ValuationHistoryPoint",
    longport::fundamental::ValuationHistoryPoint,
    [date, pe, pb, ps]
);

impl_java_class!(
    "com/longport/fundamental/ValuationComparisonItem",
    longport::fundamental::ValuationComparisonItem,
    [
        symbol,
        name,
        currency,
        market_value,
        price_close,
        pe,
        pb,
        ps,
        roe,
        eps,
        bps,
        dps,
        div_yld,
        assets,
        #[java(objarray)]
        history
    ]
);

impl_java_class!(
    "com/longport/fundamental/ValuationComparisonResponse",
    longport::fundamental::ValuationComparisonResponse,
    [
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longport/fundamental/MultiLanguageText",
    longport::fundamental::MultiLanguageText,
    [english, simplified_chinese, traditional_chinese]
);

impl_java_class!(
    "com/longport/fundamental/MacroeconomicIndicator",
    longport::fundamental::MacroeconomicIndicator,
    [
        indicator_code,
        source_org,
        country,
        name,
        adjustment_factor,
        periodicity,
        category,
        describe,
        importance,
        start_date
    ]
);

impl_java_class!(
    "com/longport/fundamental/Macroeconomic",
    longport::fundamental::Macroeconomic,
    [
        period,
        release_at,
        actual_value,
        previous_value,
        forecast_value,
        revised_value,
        next_release_at,
        unit,
        unit_prefix
    ]
);

impl_java_class!(
    "com/longport/fundamental/MacroeconomicIndicatorListResponse",
    longport::fundamental::MacroeconomicIndicatorListResponse,
    [
        #[java(objarray)]
        data,
        count
    ]
);

impl_java_class!(
    "com/longport/fundamental/MacroeconomicResponse",
    longport::fundamental::MacroeconomicResponse,
    [
        info,
        #[java(objarray)]
        data,
        count
    ]
);

// ── MarketContext: top movers / rank ──────────────────────────────

impl_java_class!(
    "com/longport/market/TopMoversStock",
    longport::market::TopMoversStock,
    [
        symbol,
        code,
        name,
        full_name,
        change,
        last_done,
        market,
        #[java(objarray)]
        labels,
        logo
    ]
);

impl_java_class!(
    "com/longport/market/TopMoversEvent",
    longport::market::TopMoversEvent,
    [timestamp, alert_reason, alert_type, stock, post]
);

impl_java_class!(
    "com/longport/market/TopMoversResponse",
    longport::market::TopMoversResponse,
    [
        #[java(objarray)]
        events,
        next_params
    ]
);

impl_java_class!(
    "com/longport/market/RankSubCategory",
    longport::market::RankSubCategory,
    [key, name, market]
);

impl_java_class!(
    "com/longport/market/RankCategory",
    longport::market::RankCategory,
    [
        key,
        name,
        #[java(objarray)]
        sub_categories
    ]
);

impl_java_class!(
    "com/longport/market/RankCategoriesResponse",
    longport::market::RankCategoriesResponse,
    [
        #[java(objarray)]
        categories
    ]
);

impl_java_class!(
    "com/longport/market/RankListItem",
    longport::market::RankListItem,
    [
        symbol,
        code,
        name,
        last_done,
        chg,
        change,
        inflow,
        market_cap,
        industry,
        pre_post_price,
        pre_post_chg,
        amplitude,
        five_day_chg,
        turnover_rate,
        volume_rate,
        pb_ttm
    ]
);

impl_java_class!(
    "com/longport/market/RankListResponse",
    longport::market::RankListResponse,
    [
        bmp,
        #[java(objarray)]
        lists
    ]
);

// ── ScreenerContext ───────────────────────────────────────────────

impl_java_class!(
    "com/longport/screener/ScreenerRecommendStrategiesResponse",
    longport::screener::ScreenerRecommendStrategiesResponse,
    [data]
);

impl_java_class!(
    "com/longport/screener/ScreenerUserStrategiesResponse",
    longport::screener::ScreenerUserStrategiesResponse,
    [data]
);

impl_java_class!(
    "com/longport/screener/ScreenerStrategyResponse",
    longport::screener::ScreenerStrategyResponse,
    [data]
);

impl_java_class!(
    "com/longport/screener/ScreenerSearchResponse",
    longport::screener::ScreenerSearchResponse,
    [data]
);

impl_java_class!(
    "com/longport/screener/ScreenerIndicatorsResponse",
    longport::screener::ScreenerIndicatorsResponse,
    [data]
);

// ── AgentContext types ─────────────────────────────────────────────

impl_java_class!(
    "com/longport/agent/Workspace",
    longport::agent::Workspace,
    [id, name, created_at, updated_at]
);

impl_java_class!(
    "com/longport/agent/WorkspacesResponse",
    longport::agent::WorkspacesResponse,
    [
        #[java(objarray)]
        workspaces
    ]
);

impl_java_class!(
    "com/longport/agent/Agent",
    longport::agent::Agent,
    [
        uid,
        name,
        description,
        mode,
        icon,
        is_published,
        published_at,
        created_at,
        updated_at
    ]
);

impl_java_class!(
    "com/longport/agent/AgentsResponse",
    longport::agent::AgentsResponse,
    [
        #[java(objarray)]
        agents,
        total
    ]
);

impl_java_class!(
    "com/longport/agent/Reference",
    longport::agent::Reference,
    [index, original_index, ref_type, id, title, url, content]
);

impl_java_class!(
    "com/longport/agent/QuestionOption",
    longport::agent::QuestionOption,
    [label, description]
);

impl_java_class!(
    "com/longport/agent/Question",
    longport::agent::Question,
    [
        question,
        #[java(objarray)]
        options,
        multi_select
    ]
);

impl_java_class!(
    "com/longport/agent/HumanInteraction",
    longport::agent::HumanInteraction,
    [
        tool_call_id,
        interrupt_id,
        interaction_type,
        tool_name,
        #[java(objarray)]
        questions,
        tool_args
    ]
);

impl_java_class!(
    "com/longport/agent/Interrupt",
    longport::agent::Interrupt,
    [
        node_id,
        tool_call_id,
        #[java(objarray)]
        questions,
        #[java(objarray)]
        interactions,
        message_id,
        chat_id
    ]
);

// The Java class is named `ConversationError` (not `AgentError`) since it
// describes a failed conversation *run*, not a JNI/transport-level failure —
// `OpenApiException` already owns that role for this SDK. `impl_java_class!`
// only cares that the two names are given independently, so no wrapper type
// is needed just to rename it.
impl_java_class!(
    "com/longport/agent/ConversationError",
    longport::agent::AgentError,
    [code, message]
);

impl_java_class!(
    "com/longport/agent/ChatStartedEvent",
    longport::agent::ChatStartedPayload,
    [chat_uid, message_id, chat_id, error, error_message]
);

// JNI-side view of `longport::agent::WorkflowStartedInputs`, the `inputs`
// sub-object of a `workflow_started` event — mapped to its own Java class
// the same way `Interrupt`/`Question` nest inside `ConversationResponse`
// above.
impl_java_class!(
    "com/longport/agent/WorkflowStartedInputs",
    longport::agent::WorkflowStartedInputs,
    [chat_id, chat_uid, message_id, query]
);

impl_java_class!(
    "com/longport/agent/WorkflowStartedEvent",
    longport::agent::WorkflowStartedPayload,
    [hit_cache, inputs, started_at, workflow_id]
);

impl_java_class!(
    "com/longport/agent/MessageEvent",
    longport::agent::MessagePayload,
    [
        text,
        message_type,
        key,
        started_at,
        stage,
        stage_title,
        stage_finished_title,
        outputs
    ]
);

/// JNI-side marker for
/// [`longport::agent::ConversationStreamEvent::Ping`], which — unlike
/// every other variant — carries no payload at all. `impl_java_class!` works
/// fine against a zero-field struct, so this is used instead of hand-writing
/// a bespoke JNI object constructor for the one variant with nothing to
/// carry. See the manual `IntoJValue` for `ConversationStreamEvent` below,
/// the only place this type is used.
pub(crate) struct PingEvent {}

impl_java_class!("com/longport/agent/PingEvent", PingEvent, []);

impl_java_class!(
    "com/longport/agent/ThinkingStartedEvent",
    longport::agent::ThinkingStartedPayload,
    [started_at]
);

impl_java_class!(
    "com/longport/agent/ThinkingFinishedEvent",
    longport::agent::ThinkingFinishedPayload,
    [finished_at, elapsed_time]
);

impl_java_class!(
    "com/longport/agent/NodeToolUseStartedEvent",
    longport::agent::NodeToolUseStartedPayload,
    [
        tool_use_id,
        tool_name,
        tool_func_name,
        tool_args,
        tips,
        #[java(objarray)]
        tip_chips,
        iteration,
        started_at
    ]
);

/// JNI-side view of [`longport::agent::NodeToolUseOutputs`], with
/// `references`/`reference_domains` normalized from `Option<Vec<_>>` down to a
/// plain `Vec` (empty when absent) — same convention as
/// [`ConversationResponse`]'s `references` field above.
pub(crate) struct NodeToolUseOutputs {
    pub(crate) references: Vec<longport::agent::Reference>,
    pub(crate) reference_domains: Vec<String>,
    pub(crate) query: Option<String>,
    pub(crate) text: Option<String>,
    pub(crate) tool_args: Option<serde_json::Value>,
    pub(crate) data: Option<serde_json::Value>,
}

impl From<longport::agent::NodeToolUseOutputs> for NodeToolUseOutputs {
    fn from(value: longport::agent::NodeToolUseOutputs) -> Self {
        Self {
            references: value.references.unwrap_or_default(),
            reference_domains: value.reference_domains.unwrap_or_default(),
            query: value.query,
            text: value.text,
            tool_args: value.tool_args,
            data: value.data,
        }
    }
}

impl_java_class!(
    "com/longport/agent/NodeToolUseOutputs",
    NodeToolUseOutputs,
    [
        #[java(objarray)]
        references,
        #[java(objarray)]
        reference_domains,
        query,
        text,
        tool_args,
        data
    ]
);

impl_java_class!(
    "com/longport/agent/NodeToolUseFinishedEvent",
    longport::agent::NodeToolUseFinishedPayload,
    [
        tool_use_id,
        status,
        error,
        elapsed_time,
        started_at,
        tool_name,
        tool_func_name,
        tool_args,
        tool_type,
        tips,
        #[java(objarray)]
        tip_chips,
        iteration,
        is_thinking,
        #[java(set_as = crate::types::NodeToolUseOutputs)]
        outputs
    ]
);

impl_java_class!(
    "com/longport/agent/SubagentStartedEvent",
    longport::agent::SubagentStartedPayload,
    [
        node_id,
        tool_use_id,
        started_at,
        goal,
        prompt,
        subagent_id,
        #[java(objarray)]
        tools
    ]
);

impl_java_class!(
    "com/longport/agent/SubagentProgressEvent",
    longport::agent::SubagentProgressPayload,
    [
        node_id,
        parent_tool_call_id,
        subagent_tool_name,
        subagent_tool_args,
        subagent_status,
        subagent_duration_ms,
        subagent_iteration,
        started_at
    ]
);

/// JNI-side view of [`longport::agent::SubagentOutputs`], with
/// `subagent_tools` normalized from `Option<Vec<_>>` down to a plain `Vec`
/// (empty when absent) — same convention as [`NodeToolUseOutputs`] above.
pub(crate) struct SubagentOutputs {
    pub(crate) goal: Option<String>,
    pub(crate) result: Option<String>,
    pub(crate) subagent_tools: Vec<serde_json::Value>,
}

impl From<longport::agent::SubagentOutputs> for SubagentOutputs {
    fn from(value: longport::agent::SubagentOutputs) -> Self {
        Self {
            goal: value.goal,
            result: value.result,
            subagent_tools: value.subagent_tools.unwrap_or_default(),
        }
    }
}

impl_java_class!(
    "com/longport/agent/SubagentOutputs",
    SubagentOutputs,
    [
        goal,
        result,
        #[java(objarray)]
        subagent_tools
    ]
);

impl_java_class!(
    "com/longport/agent/SubagentFinishedEvent",
    longport::agent::SubagentFinishedPayload,
    [
        node_id,
        tool_use_id,
        status,
        started_at,
        elapsed_time,
        error,
        #[java(set_as = crate::types::SubagentOutputs)]
        outputs
    ]
);

impl_java_class!(
    "com/longport/agent/AgentToolStartedEvent",
    longport::agent::AgentToolStartedPayload,
    [
        node_id,
        tool_use_id,
        agent_tool_name,
        title,
        started_at,
        tool_args,
        tool_name,
        tips,
        #[java(objarray)]
        tip_chips,
        is_thinking
    ]
);

impl_java_class!(
    "com/longport/agent/AgentToolProgressEvent",
    longport::agent::AgentToolProgressPayload,
    [
        node_id,
        parent_tool_call_id,
        agent_tool_name,
        inner_tool_name,
        inner_tool_args,
        status,
        duration_ms,
        started_at,
        is_thinking
    ]
);

impl_java_class!(
    "com/longport/agent/AgentToolFinishedEvent",
    longport::agent::AgentToolFinishedPayload,
    [
        node_id,
        tool_use_id,
        agent_tool_name,
        status,
        started_at,
        elapsed_time,
        error,
        tool_args,
        outputs,
        tool_type,
        tips,
        #[java(objarray)]
        tip_chips,
        is_thinking
    ]
);

/// JNI-side wrapper so that
/// [`longport::agent::ConversationStreamEvent::HumanInteractionRequired`]
/// can go through the same `impl_java_class!` machinery as every other event
/// payload, wrapping the same [`ConversationResponse`] that
/// [`WorkflowFinishedEvent`] wraps — see the manual `IntoJValue` for
/// `ConversationStreamEvent` below, which is the only place this type is
/// used.
pub(crate) struct HumanInteractionRequiredEvent {
    pub(crate) response: ConversationResponse,
}

impl_java_class!(
    "com/longport/agent/HumanInteractionRequiredEvent",
    HumanInteractionRequiredEvent,
    [response]
);

impl_java_class!(
    "com/longport/agent/QueryMaskedEvent",
    longport::agent::QueryMaskedPayload,
    [raw_query, masked_query]
);

impl_java_class!(
    "com/longport/agent/PlanChangedEvent",
    longport::agent::PlanChangedPayload,
    [node_id, started_at, outputs, tool_name]
);

impl_java_class!(
    "com/longport/agent/ContextCompressStartedEvent",
    longport::agent::ContextCompressStartedPayload,
    [started_at, inputs]
);

impl_java_class!(
    "com/longport/agent/ContextCompressFinishedEvent",
    longport::agent::ContextCompressFinishedPayload,
    [created_at, inputs, outputs]
);

impl_java_class!(
    "com/longport/agent/ChatFinishedEvent",
    longport::agent::ChatFinishedPayload,
    [chat_id, chat_uid, message_id, error, error_message]
);

/// JNI-side view of [`longport::agent::ConversationResponse`], with
/// `references`/`further_questions` normalized from `Option<Vec<_>>` down to
/// a plain `Vec` (empty when absent) so they can use the same
/// `#[java(objarray)]` convention as every other list field — mirrors how
/// `StockPosition` above collapses `Option<Decimal>`/`Option<i64>` fields
/// with `unwrap_or_default`.
pub(crate) struct ConversationResponse {
    pub(crate) chat_uid: String,
    pub(crate) message_id: String,
    pub(crate) status: longport::agent::ConversationStatus,
    pub(crate) answer: String,
    pub(crate) references: Vec<longport::agent::Reference>,
    pub(crate) further_questions: Vec<String>,
    pub(crate) elapsed_time: f64,
    pub(crate) interrupt: Option<longport::agent::Interrupt>,
    pub(crate) error: Option<longport::agent::AgentError>,
}

impl From<longport::agent::ConversationResponse> for ConversationResponse {
    fn from(value: longport::agent::ConversationResponse) -> Self {
        Self {
            chat_uid: value.chat_uid,
            message_id: value.message_id,
            status: value.status,
            answer: value.answer,
            references: value.references.unwrap_or_default(),
            further_questions: value.further_questions.unwrap_or_default(),
            elapsed_time: value.elapsed_time,
            interrupt: value.interrupt,
            error: value.error,
        }
    }
}

impl_java_class!(
    "com/longport/agent/ConversationResponse",
    ConversationResponse,
    [
        chat_uid,
        message_id,
        status,
        answer,
        #[java(objarray)]
        references,
        #[java(objarray)]
        further_questions,
        elapsed_time,
        interrupt,
        error
    ]
);

/// JNI-side wrapper so that
/// [`longport::agent::ConversationStreamEvent::WorkflowFinished`] can go
/// through the same `impl_java_class!` machinery as every other event
/// payload — see the manual `IntoJValue` for `ConversationStreamEvent` below,
/// which is the only place this type is used.
pub(crate) struct WorkflowFinishedEvent {
    pub(crate) response: ConversationResponse,
}

impl_java_class!(
    "com/longport/agent/WorkflowFinishedEvent",
    WorkflowFinishedEvent,
    [response]
);

impl_java_class!(
    "com/longport/agent/ChatTitleUpdatedEvent",
    longport::agent::ChatTitleUpdatedPayload,
    [chat_id, chat_uid, source, title, updated_at]
);

/// JNI-side wrapper for
/// [`longport::agent::ConversationStreamEvent::Other`], carrying the SSE
/// envelope's `event` type name alongside the raw event JSON as a string
/// (same convention as `AlertItem.valueMap`).
pub(crate) struct OtherEvent {
    pub(crate) event: String,
    pub(crate) json: serde_json::Value,
}

impl_java_class!("com/longport/agent/OtherEvent", OtherEvent, [event, json]);

/// `ConversationStreamEvent` is an enum-with-payload, so unlike every type
/// above it can't go through `impl_java_class!` (which only knows how to
/// build a single Java class by reflecting named struct fields onto it).
/// Instead each variant is modeled as its own Java subclass of the
/// `ConversationStreamEvent` sealed-style hierarchy
/// (`ChatStartedEvent`/`WorkflowStartedEvent`/`MessageEvent`/`PingEvent`/
/// `ThinkingStartedEvent`/`ThinkingFinishedEvent`/`NodeToolUseStartedEvent`/
/// `NodeToolUseFinishedEvent`/`SubagentStartedEvent`/`SubagentProgressEvent`/
/// `SubagentFinishedEvent`/`AgentToolStartedEvent`/`AgentToolProgressEvent`/
/// `AgentToolFinishedEvent`/`HumanInteractionRequiredEvent`/
/// `QueryMaskedEvent`/`PlanChangedEvent`/`ContextCompressStartedEvent`/
/// `ContextCompressFinishedEvent`/`ChatFinishedEvent`/`WorkflowFinishedEvent`/
/// `ChatTitleUpdatedEvent`/`OtherEvent`), and this manual `IntoJValue` picks
/// the right one — the resulting object reference is passed to
/// `Flow.Subscriber.onNext(Object)` as-is (generics are erased on the Java
/// side, so any subclass of the sealed base is a valid argument there).
impl crate::types::IntoJValue for longport::agent::ConversationStreamEvent {
    fn into_jvalue<'a>(
        self,
        env: &mut jni::JNIEnv<'a>,
    ) -> jni::errors::Result<jni::objects::JValueOwned<'a>> {
        use longport::agent::ConversationStreamEvent::*;
        match self {
            ChatStarted(payload) => payload.into_jvalue(env),
            WorkflowStarted(payload) => payload.into_jvalue(env),
            Message(payload) => payload.into_jvalue(env),
            Ping => PingEvent {}.into_jvalue(env),
            ThinkingStarted(payload) => payload.into_jvalue(env),
            ThinkingFinished(payload) => payload.into_jvalue(env),
            NodeToolUseStarted(payload) => payload.into_jvalue(env),
            NodeToolUseFinished(payload) => payload.into_jvalue(env),
            SubagentStarted(payload) => payload.into_jvalue(env),
            SubagentProgress(payload) => payload.into_jvalue(env),
            SubagentFinished(payload) => payload.into_jvalue(env),
            AgentToolStarted(payload) => payload.into_jvalue(env),
            AgentToolProgress(payload) => payload.into_jvalue(env),
            AgentToolFinished(payload) => payload.into_jvalue(env),
            HumanInteractionRequired(resp) => HumanInteractionRequiredEvent {
                response: ConversationResponse::from(resp),
            }
            .into_jvalue(env),
            QueryMasked(payload) => payload.into_jvalue(env),
            PlanChanged(payload) => payload.into_jvalue(env),
            ContextCompressStarted(payload) => payload.into_jvalue(env),
            ContextCompressFinished(payload) => payload.into_jvalue(env),
            ChatFinished(payload) => payload.into_jvalue(env),
            WorkflowFinished(resp) => WorkflowFinishedEvent {
                response: ConversationResponse::from(resp),
            }
            .into_jvalue(env),
            ChatTitleUpdated(payload) => payload.into_jvalue(env),
            Other { event, data } => OtherEvent { event, json: data }.into_jvalue(env),
        }
    }
}
