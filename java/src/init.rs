use std::sync::OnceLock;

use jni::{
    JNIEnv,
    descriptors::Desc,
    objects::{GlobalRef, JClass, JValue},
};

use crate::types::ClassLoader;

pub(crate) static INTEGER_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static LONG_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static STRING_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static DECIMAL_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static TIME_INSTANT_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static TIME_OFFSETDATETIME_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static TIME_LOCALDATE_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static TIME_LOCALTIME_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static TIME_LOCALDATETIME_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static TIME_ZONE_ID: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static DERIVATIVE_TYPE_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static OPENAPI_EXCEPTION_CLASS: OnceLock<GlobalRef> = OnceLock::new();

fn init_timezone_id(env: &mut JNIEnv) {
    let utc = env.new_string("UTC").unwrap();
    let zone_id = env
        .call_static_method(
            "java/time/ZoneId",
            "of",
            "(Ljava/lang/String;)Ljava/time/ZoneId;",
            &[JValue::from(&utc)],
        )
        .expect("create zone id");
    let _ = TIME_ZONE_ID.set(env.new_global_ref(zone_id.l().unwrap()).unwrap());
}

macro_rules! init_class {
    ($env:expr, $(($id:ident, $ty:literal)),*) => {
        $(
        let cls = Desc::<JClass>::lookup($ty, &mut $env).expect($ty);
        let _ = $id.set($env.new_global_ref::<&JClass>(&*cls).unwrap());
        )*
    };
}

macro_rules! init_class_by_classloader {
    ($env:expr, $($id:ty),*) => {
        $(
            <$id>::init(&mut $env);
        )*
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_longport_SdkNative_init<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
) {
    init_class!(
        env,
        (INTEGER_CLASS, "java/lang/Integer"),
        (LONG_CLASS, "java/lang/Long"),
        (STRING_CLASS, "java/lang/String"),
        (DECIMAL_CLASS, "java/math/BigDecimal"),
        (TIME_INSTANT_CLASS, "java/time/Instant"),
        (TIME_OFFSETDATETIME_CLASS, "java/time/OffsetDateTime"),
        (TIME_LOCALDATE_CLASS, "java/time/LocalDate"),
        (TIME_LOCALTIME_CLASS, "java/time/LocalTime"),
        (TIME_LOCALDATETIME_CLASS, "java/time/LocalDateTime"),
        (DERIVATIVE_TYPE_CLASS, "com/longport/quote/DerivativeType"),
        (OPENAPI_EXCEPTION_CLASS, "com/longport/OpenApiException")
    );

    init_timezone_id(&mut env);

    // enum types
    init_class_by_classloader!(
        env,
        longport::SimpleErrorKind,
        longport::Language,
        longport::PushCandlestickMode,
        longport::Market,
        longport::quote::TradeStatus,
        longport::quote::TradeSession,
        longport::quote::TradeDirection,
        longport::quote::OptionType,
        longport::quote::OptionDirection,
        longport::quote::WarrantType,
        longport::quote::WarrantStatus,
        longport::quote::SortOrderType,
        longport::quote::WarrantSortBy,
        longport::quote::FilterWarrantExpiryDate,
        longport::quote::FilterWarrantInOutBoundsType,
        longport::quote::Period,
        longport::quote::AdjustType,
        longport::quote::SecurityBoard,
        longport::quote::SecuritiesUpdateMode,
        longport::quote::CalcIndex,
        longport::quote::SecurityListCategory,
        longport::quote::TradeSessions,
        longport::quote::Granularity,
        longport::trade::OrderSide,
        longport::trade::OrderType,
        longport::trade::OrderStatus,
        longport::trade::OrderTag,
        longport::trade::TriggerStatus,
        longport::trade::TopicType,
        longport::trade::TimeInForceType,
        longport::trade::OutsideRTH,
        longport::trade::BalanceType,
        longport::trade::CashFlowDirection,
        longport::trade::CommissionFreeStatus,
        longport::trade::DeductionStatus,
        longport::trade::ChargeCategoryCode,
        longport::quote::PinnedMode,
        longport::portfolio::types::FlowDirection,
        longport::portfolio::types::AssetType,
        longport::fundamental::types::InstitutionRecommend,
        longport::fundamental::types::FinancialReportKind,
        longport::fundamental::types::FinancialReportPeriod,
        longport::market::types::BrokerHoldingPeriod,
        longport::market::types::AhPremiumPeriod,
        longport::dca::types::DCAFrequency,
        longport::dca::types::DCAStatus,
        longport::alert::types::AlertCondition,
        longport::alert::types::AlertFrequency,
        longport::calendar::types::CalendarCategory
    );

    // classes
    init_class_by_classloader!(
        env,
        longport::quote::Trade,
        longport::quote::Brokers,
        longport::quote::Depth,
        longport::quote::Subscription,
        longport::quote::PushQuote,
        longport::quote::PushDepth,
        longport::quote::PushBrokers,
        longport::quote::PushTrades,
        longport::quote::PushCandlestick,
        longport::quote::SecurityStaticInfo,
        longport::quote::PrePostQuote,
        longport::quote::SecurityQuote,
        longport::quote::OptionQuote,
        longport::quote::WarrantQuote,
        longport::quote::SecurityDepth,
        longport::quote::SecurityBrokers,
        longport::quote::ParticipantInfo,
        longport::quote::IntradayLine,
        longport::quote::Candlestick,
        longport::quote::StrikePriceInfo,
        longport::quote::IssuerInfo,
        longport::quote::WarrantInfo,
        longport::quote::MarketTradingSession,
        longport::quote::TradingSessionInfo,
        longport::quote::MarketTradingDays,
        longport::quote::CapitalFlowLine,
        longport::quote::CapitalDistribution,
        longport::quote::CapitalDistributionResponse,
        crate::types::SecurityCalcIndex,
        longport::quote::WatchlistGroup,
        longport::quote::WatchlistSecurity,
        crate::types::CreateWatchlistGroupResponse,
        longport::quote::RealtimeQuote,
        longport::quote::Security,
        longport::quote::QuotePackageDetail,
        longport::quote::MarketTemperature,
        longport::quote::HistoryMarketTemperatureResponse,
        longport::quote::FilingItem,
        longport::trade::PushOrderChanged,
        longport::trade::Execution,
        longport::trade::Order,
        longport::trade::SubmitOrderResponse,
        longport::trade::CashInfo,
        longport::trade::FrozenTransactionFee,
        longport::trade::AccountBalance,
        longport::trade::CashFlow,
        longport::trade::FundPositionsResponse,
        longport::trade::FundPositionChannel,
        longport::trade::FundPosition,
        crate::types::StockPositionsResponse,
        crate::types::StockPositionChannel,
        crate::types::StockPosition,
        longport::trade::MarginRatio,
        longport::trade::OrderHistoryDetail,
        longport::trade::OrderChargeFee,
        longport::trade::OrderChargeItem,
        longport::trade::OrderChargeDetail,
        longport::trade::OrderDetail,
        longport::trade::EstimateMaxPurchaseQuantityResponse,
        longport::content::TopicItem,
        longport::content::NewsItem,
        longport::content::TopicAuthor,
        longport::content::TopicImage,
        longport::content::OwnedTopic,
        longport::dca::DcaPlan,
        longport::dca::DcaList,
        longport::dca::DcaStats,
        longport::sharelist::SharelistStock,
        longport::sharelist::SharelistScopes,
        longport::sharelist::SharelistInfo,
        longport::sharelist::SharelistList,
        longport::sharelist::SharelistDetail,
        // DCAContext (partial - types with serde_json::Value use JSON)
        longport::dca::DcaHistoryRecord,
        longport::dca::DcaHistoryResponse,
        longport::dca::DcaSupportInfo,
        longport::dca::DcaSupportList,
        longport::dca::DcaCalcDateResult,
        // AlertContext - list returns JSON via AlertList IntoJValue (serde_json)
        longport::dca::DcaPlan,
        longport::dca::DcaList,
        longport::dca::DcaStats,
        longport::sharelist::SharelistStock,
        longport::sharelist::SharelistScopes,
        longport::sharelist::SharelistInfo,
        longport::sharelist::SharelistList,
        longport::sharelist::SharelistDetail,
        // DCAContext (partial - types with serde_json::Value use JSON)
        longport::dca::DcaHistoryRecord,
        longport::dca::DcaHistoryResponse,
        longport::dca::DcaSupportInfo,
        longport::dca::DcaSupportList,
        // AlertContext
        longport::alert::AlertItem,
        longport::alert::AlertSymbolGroup,
        longport::alert::AlertList,
        // CalendarContext
        longport::calendar::CalendarEventsResponse,
        longport::calendar::CalendarDateGroup,
        longport::calendar::CalendarEventInfo,
        longport::calendar::CalendarDataKv,
        // PortfolioContext
        longport::portfolio::ExchangeRates,
        longport::portfolio::ExchangeRate,
        longport::portfolio::ProfitAnalysis,
        longport::portfolio::ProfitAnalysisSummary,
        longport::portfolio::ProfitSummaryBreakdown,
        longport::portfolio::ProfitSummaryInfo,
        longport::portfolio::ProfitAnalysisSublist,
        longport::portfolio::ProfitAnalysisItem,
        longport::portfolio::ProfitAnalysisDetail,
        longport::portfolio::ProfitDetails,
        longport::portfolio::ProfitDetailEntry,
        longport::portfolio::ProfitAnalysisByMarketItem,
        longport::portfolio::ProfitAnalysisByMarket,
        // MarketContext
        longport::market::MarketStatusResponse,
        longport::market::MarketTimeItem,
        longport::market::BrokerHoldingTop,
        longport::market::BrokerHoldingEntry,
        longport::market::BrokerHoldingDetail,
        longport::market::BrokerHoldingDetailItem,
        longport::market::BrokerHoldingChanges,
        longport::market::BrokerHoldingDailyHistory,
        longport::market::BrokerHoldingDailyItem,
        longport::market::AhPremiumKlines,
        longport::market::AhPremiumIntraday,
        longport::market::AhPremiumKline,
        longport::market::TradeStatsResponse,
        longport::market::TradeStatistics,
        longport::market::TradePriceLevel,
        longport::market::AnomalyResponse,
        longport::market::AnomalyItem,
        longport::market::IndexConstituents,
        longport::market::ConstituentStock,
        longport::dca::DcaPlan,
        longport::dca::DcaList,
        longport::dca::DcaStats,
        longport::sharelist::SharelistStock,
        longport::sharelist::SharelistScopes,
        longport::sharelist::SharelistInfo,
        longport::sharelist::SharelistList,
        longport::sharelist::SharelistDetail,
        // DCAContext (partial - types with serde_json::Value use JSON)
        longport::dca::DcaHistoryRecord,
        longport::dca::DcaHistoryResponse,
        longport::dca::DcaSupportInfo,
        longport::dca::DcaSupportList,
        longport::dca::DcaCalcDateResult,
        // AlertContext - list returns JSON via AlertList IntoJValue (serde_json)
        longport::dca::DcaPlan,
        longport::dca::DcaList,
        longport::dca::DcaStats,
        longport::sharelist::SharelistStock,
        longport::sharelist::SharelistScopes,
        longport::sharelist::SharelistInfo,
        longport::sharelist::SharelistList,
        longport::sharelist::SharelistDetail,
        // DCAContext (partial - types with serde_json::Value use JSON)
        longport::dca::DcaHistoryRecord,
        longport::dca::DcaHistoryResponse,
        longport::dca::DcaSupportInfo,
        longport::dca::DcaSupportList,
        // AlertContext
        longport::alert::AlertItem,
        longport::alert::AlertSymbolGroup,
        longport::alert::AlertList,
        // CalendarContext
        longport::calendar::CalendarEventsResponse,
        longport::calendar::CalendarDateGroup,
        longport::calendar::CalendarEventInfo,
        longport::calendar::CalendarDataKv,
        // PortfolioContext
        longport::portfolio::ExchangeRates,
        longport::portfolio::ExchangeRate,
        longport::portfolio::ProfitAnalysis,
        longport::portfolio::ProfitAnalysisSummary,
        longport::portfolio::ProfitSummaryBreakdown,
        longport::portfolio::ProfitSummaryInfo,
        longport::portfolio::ProfitAnalysisSublist,
        longport::portfolio::ProfitAnalysisItem,
        longport::portfolio::ProfitAnalysisDetail,
        longport::portfolio::ProfitDetails,
        longport::portfolio::ProfitDetailEntry,
        // FundamentalContext
        longport::fundamental::FinancialReports,
        longport::fundamental::DividendList,
        longport::fundamental::DividendItem,
        longport::fundamental::InstitutionRating,
        longport::fundamental::InstitutionRatingLatest,
        longport::fundamental::RatingEvaluate,
        longport::fundamental::RatingTarget,
        longport::fundamental::InstitutionRatingSummary,
        longport::fundamental::RatingSummaryEvaluate,
        longport::fundamental::InstitutionRatingDetail,
        longport::fundamental::InstitutionRatingDetailEvaluate,
        longport::fundamental::InstitutionRatingDetailEvaluateItem,
        longport::fundamental::InstitutionRatingDetailTarget,
        longport::fundamental::InstitutionRatingDetailTargetItem,
        longport::fundamental::ForecastEps,
        longport::fundamental::ForecastEpsItem,
        longport::fundamental::FinancialConsensus,
        longport::fundamental::ConsensusReport,
        longport::fundamental::ConsensusDetail,
        longport::fundamental::ValuationData,
        longport::fundamental::ValuationMetricsData,
        longport::fundamental::ValuationMetricData,
        longport::fundamental::ValuationPoint,
        longport::fundamental::ValuationHistoryResponse,
        longport::fundamental::ValuationHistoryData,
        longport::fundamental::ValuationHistoryMetrics,
        longport::fundamental::ValuationHistoryMetric,
        longport::fundamental::IndustryValuationList,
        longport::fundamental::IndustryValuationItem,
        longport::fundamental::IndustryValuationHistory,
        longport::fundamental::IndustryValuationDist,
        longport::fundamental::ValuationDist,
        longport::fundamental::CompanyOverview,
        longport::fundamental::ExecutiveList,
        longport::fundamental::ExecutiveGroup,
        longport::fundamental::Professional,
        longport::fundamental::ShareholderList,
        longport::fundamental::Shareholder,
        longport::fundamental::ShareholderStock,
        longport::fundamental::FundHolders,
        longport::fundamental::FundHolder,
        longport::fundamental::CorpActions,
        longport::fundamental::CorpActionLive,
        longport::fundamental::CorpActionItem,
        longport::fundamental::InvestRelations,
        longport::fundamental::InvestSecurity,
        longport::fundamental::OperatingList,
        longport::fundamental::OperatingItem,
        longport::fundamental::OperatingFinancial,
        longport::fundamental::OperatingIndicator,
        // QuoteContext extensions
        longport::quote::ShortPositionsItem,
        longport::quote::ShortPositionsResponse,
        longport::quote::ShortTradesItem,
        longport::quote::ShortTradesResponse,
        longport::quote::OptionVolumeStats,
        longport::quote::OptionVolumeDaily,
        longport::quote::OptionVolumeDailyStat,
        // FundamentalContext: BuybackData
        longport::fundamental::RecentBuybacks,
        longport::fundamental::BuybackHistoryItem,
        longport::fundamental::BuybackRatios,
        longport::fundamental::BuybackData,
        // FundamentalContext: StockRatings
        longport::fundamental::RatingIndicator,
        longport::fundamental::RatingLeafIndicator,
        longport::fundamental::RatingSubIndicatorGroup,
        longport::fundamental::RatingCategory,
        longport::fundamental::StockRatings,
        // FundamentalContext: shareholders / valuation comparison
        longport::fundamental::ShareholderTopResponse,
        longport::fundamental::ShareholderDetailResponse,
        longport::fundamental::ValuationHistoryPoint,
        longport::fundamental::ValuationComparisonItem,
        longport::fundamental::ValuationComparisonResponse,
        // MarketContext: top movers / rank
        longport::market::TopMoversStock,
        longport::market::TopMoversEvent,
        longport::market::TopMoversResponse,
        longport::market::RankCategoriesResponse,
        longport::market::RankListItem,
        longport::market::RankListResponse,
        // ScreenerContext
        longport::screener::ScreenerRecommendStrategiesResponse,
        longport::screener::ScreenerUserStrategiesResponse,
        longport::screener::ScreenerStrategyResponse,
        longport::screener::ScreenerSearchResponse,
        longport::screener::ScreenerIndicatorsResponse,
        // PortfolioContext: ProfitAnalysisFlows
        longport::portfolio::FlowItem,
        longport::portfolio::ProfitAnalysisFlows,
        // DCAContext
        longport::dca::DcaCreateResult
    );
}
