mod context;
mod push;
mod types;

use pyo3::prelude::*;

pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    parent.add_class::<types::DerivativeType>()?;
    parent.add_class::<types::TradeStatus>()?;
    parent.add_class::<types::TradeSession>()?;
    parent.add_class::<types::SubType>()?;
    parent.add_class::<types::TradeDirection>()?;
    parent.add_class::<types::OptionType>()?;
    parent.add_class::<types::Period>()?;
    parent.add_class::<types::AdjustType>()?;
    parent.add_class::<types::SecurityStaticInfo>()?;
    parent.add_class::<types::PrePostQuote>()?;
    parent.add_class::<types::SecurityQuote>()?;
    parent.add_class::<types::OptionQuote>()?;
    parent.add_class::<types::WarrantQuote>()?;
    parent.add_class::<types::Depth>()?;
    parent.add_class::<types::SecurityDepth>()?;
    parent.add_class::<types::Brokers>()?;
    parent.add_class::<types::SecurityBrokers>()?;
    parent.add_class::<types::ParticipantInfo>()?;
    parent.add_class::<types::Trade>()?;
    parent.add_class::<types::IntradayLine>()?;
    parent.add_class::<types::Candlestick>()?;
    parent.add_class::<types::StrikePriceInfo>()?;
    parent.add_class::<types::IssuerInfo>()?;
    parent.add_class::<types::TradingSessionInfo>()?;
    parent.add_class::<types::MarketTradingSession>()?;
    parent.add_class::<types::RealtimeQuote>()?;
    parent.add_class::<types::PushQuote>()?;
    parent.add_class::<types::PushDepth>()?;
    parent.add_class::<types::PushBrokers>()?;
    parent.add_class::<types::PushTrades>()?;
    parent.add_class::<types::PushCandlestick>()?;
    parent.add_class::<types::CalcIndex>()?;
    parent.add_class::<types::SecurityCalcIndex>()?;
    parent.add_class::<types::WatchlistSecurity>()?;
    parent.add_class::<types::WatchlistGroup>()?;
    parent.add_class::<types::SecuritiesUpdateMode>()?;
    parent.add_class::<types::WarrantInfo>()?;
    parent.add_class::<types::WarrantStatus>()?;
    parent.add_class::<types::WarrantType>()?;
    parent.add_class::<types::SortOrderType>()?;
    parent.add_class::<types::WarrantSortBy>()?;
    parent.add_class::<types::FilterWarrantExpiryDate>()?;
    parent.add_class::<types::FilterWarrantInOutBoundsType>()?;
    parent.add_class::<types::Security>()?;
    parent.add_class::<types::SecurityListCategory>()?;
    parent.add_class::<types::TradeSessions>()?;
    parent.add_class::<types::MarketTemperature>()?;
    parent.add_class::<types::Granularity>()?;
    parent.add_class::<types::HistoryMarketTemperatureResponse>()?;
    parent.add_class::<types::MarketTradingDays>()?;
    parent.add_class::<types::CapitalFlowLine>()?;
    parent.add_class::<types::CapitalDistributionResponse>()?;
    parent.add_class::<types::SecurityBoard>()?;

    parent.add_class::<context::QuoteContext>()?;
    Ok(())
}
