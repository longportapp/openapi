use std::borrow::Cow;

use jni::{
    JNIEnv,
    errors::Result,
    objects::{JObject, JValueOwned},
};
use longport_java_macros::impl_java_enum;

use crate::{
    init::DERIVATIVE_TYPE_CLASS,
    types::{ClassLoader, FromJValue, IntoJValue, JSignature},
};

impl_java_enum!(
    "com/longport/ErrorKind",
    longport::SimpleErrorKind,
    [Http, OpenApi, Other, OAuth]
);

impl_java_enum!(
    "com/longport/Language",
    longport::Language,
    [ZH_CN, ZH_HK, EN]
);
impl_java_enum!(
    "com/longport/forex/ForexOrderStatus",
    longport::forex::ForexOrderStatus,
    [Processing, Success, Failed]
);

impl_java_enum!(
    "com/longport/PushCandlestickMode",
    longport::PushCandlestickMode,
    [Realtime, Confirmed]
);

impl_java_enum!(
    "com/longport/Market",
    longport::Market,
    [Unknown, US, HK, CN, SG, Crypto]
);

impl_java_enum!(
    "com/longport/quote/TradeStatus",
    longport::quote::TradeStatus,
    [
        Normal,
        Halted,
        Delisted,
        Fuse,
        PrepareList,
        CodeMoved,
        ToBeOpened,
        SplitStockHalts,
        Expired,
        WarrantPrepareList,
        SuspendTrade
    ]
);

impl_java_enum!(
    "com/longport/quote/TradeSession",
    longport::quote::TradeSession,
    [Intraday, Pre, Post, Overnight]
);

impl_java_enum!(
    "com/longport/quote/TradeDirection",
    longport::quote::TradeDirection,
    [Neutral, Down, Up]
);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum DerivativeType {
    Option,
    Warrant,
}

pub(crate) struct DerivativeTypes(Vec<DerivativeType>);

impl From<longport::quote::DerivativeType> for DerivativeTypes {
    fn from(ty: longport::quote::DerivativeType) -> Self {
        let mut res = Vec::new();
        if ty.contains(longport::quote::DerivativeType::OPTION) {
            res.push(DerivativeType::Option);
        }
        if ty.contains(longport::quote::DerivativeType::WARRANT) {
            res.push(DerivativeType::Warrant);
        }
        DerivativeTypes(res)
    }
}

impl JSignature for DerivativeTypes {
    fn signature() -> Cow<'static, str> {
        concat!("[L", "com/longport/quote/DerivativeType", ";").into()
    }
}

impl IntoJValue for DerivativeTypes {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        let cls = DERIVATIVE_TYPE_CLASS.get().unwrap();
        let array = env.new_object_array(self.0.len() as i32, cls, JObject::null())?;
        for (i, obj) in self.0.into_iter().enumerate() {
            let value = match obj {
                DerivativeType::Option => env.get_static_field(
                    cls,
                    "Option",
                    concat!("L", "com/longport/quote/DerivativeType", ";"),
                )?,
                DerivativeType::Warrant => env.get_static_field(
                    cls,
                    "Warrant",
                    concat!("L", "com/longport/quote/DerivativeType", ";"),
                )?,
            };
            env.set_object_array_element(&array, i as i32, value.l()?)?;
        }
        Ok(array.into())
    }
}

impl_java_enum!(
    "com/longport/quote/OptionType",
    longport::quote::OptionType,
    [Unknown, American, Europe]
);

impl_java_enum!(
    "com/longport/quote/OptionDirection",
    longport::quote::OptionDirection,
    [Unknown, Put, Call]
);

impl_java_enum!(
    "com/longport/quote/WarrantType",
    longport::quote::WarrantType,
    [Unknown, Call, Put, Bull, Bear, Inline]
);

impl_java_enum!(
    "com/longport/quote/Period",
    longport::quote::Period,
    [
        #[java(remote = "UnknownPeriod")]
        Unknown,
        #[java(remote = "OneMinute")]
        Min_1,
        #[java(remote = "TwoMinute")]
        Min_2,
        #[java(remote = "ThreeMinute")]
        Min_3,
        #[java(remote = "FiveMinute")]
        Min_5,
        #[java(remote = "TenMinute")]
        Min_10,
        #[java(remote = "FifteenMinute")]
        Min_15,
        #[java(remote = "TwentyMinute")]
        Min_20,
        #[java(remote = "ThirtyMinute")]
        Min_30,
        #[java(remote = "FortyFiveMinute")]
        Min_45,
        #[java(remote = "SixtyMinute")]
        Min_60,
        #[java(remote = "TwoHour")]
        Min_120,
        #[java(remote = "ThreeHour")]
        Min_180,
        #[java(remote = "FourHour")]
        Min_240,
        Day,
        Week,
        Month,
        #[java(remote = "Quarter")]
        Quarter,
        Year,
    ]
);

impl_java_enum!(
    "com/longport/quote/AdjustType",
    longport::quote::AdjustType,
    [NoAdjust, ForwardAdjust]
);

impl_java_enum!(
    "com/longport/quote/SecurityBoard",
    longport::quote::SecurityBoard,
    [
        Unknown,
        USMain,
        USPink,
        USDJI,
        USNSDQ,
        USSector,
        USOption,
        USOptionS,
        HKEquity,
        HKPreIPO,
        HKWarrant,
        HKHS,
        HKSector,
        SHMainConnect,
        SHMainNonConnect,
        SHSTAR,
        CNIX,
        CNSector,
        SZMainConnect,
        SZMainNonConnect,
        SZGEMConnect,
        SZGEMNonConnect,
        SGMain,
        STI,
        SGSector,
        SPXIndex,
        VIXIndex,
    ]
);

impl_java_enum!(
    "com/longport/quote/SecuritiesUpdateMode",
    longport::quote::SecuritiesUpdateMode,
    [Add, Remove, Replace]
);

impl_java_enum!(
    "com/longport/quote/PinnedMode",
    longport::quote::PinnedMode,
    [Add, Remove]
);

impl_java_enum!(
    "com/longport/quote/CalcIndex",
    longport::quote::CalcIndex,
    [
        LastDone,
        ChangeValue,
        ChangeRate,
        Volume,
        Turnover,
        YtdChangeRate,
        TurnoverRate,
        TotalMarketValue,
        CapitalFlow,
        Amplitude,
        VolumeRatio,
        PeTtmRatio,
        PbRatio,
        DividendRatioTtm,
        FiveDayChangeRate,
        TenDayChangeRate,
        HalfYearChangeRate,
        FiveMinutesChangeRate,
        ExpiryDate,
        StrikePrice,
        UpperStrikePrice,
        LowerStrikePrice,
        OutstandingQty,
        OutstandingRatio,
        Premium,
        ItmOtm,
        ImpliedVolatility,
        WarrantDelta,
        CallPrice,
        ToCallPrice,
        EffectiveLeverage,
        LeverageRatio,
        ConversionRatio,
        BalancePoint,
        OpenInterest,
        Delta,
        Gamma,
        Theta,
        Vega,
        Rho,
    ]
);

impl_java_enum!(
    "com/longport/quote/WarrantStatus",
    longport::quote::WarrantStatus,
    [Suspend, PrepareList, Normal]
);

impl_java_enum!(
    "com/longport/quote/SortOrderType",
    longport::quote::SortOrderType,
    [Ascending, Descending]
);

impl_java_enum!(
    "com/longport/quote/WarrantSortBy",
    longport::quote::WarrantSortBy,
    [
        LastDone,
        ChangeRate,
        ChangeValue,
        Volume,
        Turnover,
        ExpiryDate,
        StrikePrice,
        UpperStrikePrice,
        LowerStrikePrice,
        OutstandingQuantity,
        OutstandingRatio,
        Premium,
        ItmOtm,
        ImpliedVolatility,
        Delta,
        CallPrice,
        ToCallPrice,
        EffectiveLeverage,
        LeverageRatio,
        ConversionRatio,
        BalancePoint,
        Status,
    ]
);

impl_java_enum!(
    "com/longport/quote/FilterWarrantExpiryDate",
    longport::quote::FilterWarrantExpiryDate,
    [LT_3, Between_3_6, Between_6_12, GT_12]
);

impl_java_enum!(
    "com/longport/quote/FilterWarrantInOutBoundsType",
    longport::quote::FilterWarrantInOutBoundsType,
    [In, Out]
);

impl_java_enum!(
    "com/longport/quote/SecurityListCategory",
    longport::quote::SecurityListCategory,
    [Overnight]
);

impl_java_enum!(
    "com/longport/quote/TradeSessions",
    longport::quote::TradeSessions,
    [Intraday, All]
);

impl_java_enum!(
    "com/longport/quote/Granularity",
    longport::quote::Granularity,
    [Unknown, Daily, Weekly, Monthly]
);

impl_java_enum!(
    "com/longport/trade/OrderSide",
    longport::trade::OrderSide,
    [Unknown, Buy, Sell]
);

impl_java_enum!(
    "com/longport/trade/OrderType",
    longport::trade::OrderType,
    [
        Unknown, LO, ELO, MO, AO, ALO, ODD, LIT, MIT, TSLPAMT, TSLPPCT, TSMAMT, TSMPCT, SLO
    ]
);

impl_java_enum!(
    "com/longport/trade/OrderStatus",
    longport::trade::OrderStatus,
    [
        Unknown,
        NotReported,
        ReplacedNotReported,
        ProtectedNotReported,
        VarietiesNotReported,
        Filled,
        WaitToNew,
        New,
        WaitToReplace,
        PendingReplace,
        Replaced,
        PartialFilled,
        WaitToCancel,
        PendingCancel,
        Rejected,
        Canceled,
        Expired,
        PartialWithdrawal,
    ]
);

impl_java_enum!(
    "com/longport/trade/OrderTag",
    longport::trade::OrderTag,
    [Unknown, Normal, LongTerm, Grey,]
);

impl_java_enum!(
    "com/longport/trade/TriggerStatus",
    longport::trade::TriggerStatus,
    [Unknown, Deactive, Active, Released]
);

impl_java_enum!(
    "com/longport/trade/TopicType",
    longport::trade::TopicType,
    [Private]
);

impl_java_enum!(
    "com/longport/trade/TimeInForceType",
    longport::trade::TimeInForceType,
    [Unknown, Day, GoodTilCanceled, GoodTilDate]
);

impl_java_enum!(
    "com/longport/trade/OutsideRTH",
    longport::trade::OutsideRTH,
    [Unknown, RTHOnly, AnyTime, Overnight, OptionPreMarket]
);

impl_java_enum!(
    "com/longport/trade/BalanceType",
    longport::trade::BalanceType,
    [Unknown, Cash, Stock, Fund]
);

impl_java_enum!(
    "com/longport/trade/CashFlowDirection",
    longport::trade::CashFlowDirection,
    [Unknown, Out, In]
);

impl_java_enum!(
    "com/longport/trade/CommissionFreeStatus",
    longport::trade::CommissionFreeStatus,
    [Unknown, None, Calculated, Pending, Ready]
);

impl_java_enum!(
    "com/longport/trade/DeductionStatus",
    longport::trade::DeductionStatus,
    [Unknown, None, NoData, Pending, Done]
);

impl_java_enum!(
    "com/longport/trade/ChargeCategoryCode",
    longport::trade::ChargeCategoryCode,
    [Unknown, Broker, Third]
);

impl_java_enum!(
    "com/longport/trade/AttachedOrderType",
    longport::trade::AttachedOrderType,
    [
        #[java(remote = Unknown)]
        UNKNOWN,
        #[java(remote = ProfitTaker)]
        PROFIT_TAKER,
        #[java(remote = StopLoss)]
        STOP_LOSS,
        #[java(remote = Bracket)]
        BRACKET,
    ]
);

impl_java_enum!(
    "com/longport/trade/MultiLegStrategy",
    longport::trade::MultiLegStrategy,
    [
        Unknown,
        CoveredCall,
        CoveredPut,
        VerticalCallSpread,
        VerticalPutSpread,
        Collar,
        Straddle,
        Strangle,
        CalendarCallSpread,
        CalendarPutSpread
    ]
);

impl_java_enum!(
    "com/longport/trade/MultiLegPosition",
    longport::trade::MultiLegPosition,
    [Unknown, Long, Short]
);

impl_java_enum!(
    "com/longport/trade/ContractDirection",
    longport::trade::ContractDirection,
    [Unknown, Call, Put]
);

impl_java_enum!(
    "com/longport/alert/AlertCondition",
    longport::alert::types::AlertCondition,
    [PriceRise, PriceFall, PercentRise, PercentFall]
);
impl_java_enum!(
    "com/longport/alert/AlertFrequency",
    longport::alert::types::AlertFrequency,
    [Daily, EveryTime, Once]
);
impl_java_enum!(
    "com/longport/dca/DCAFrequency",
    longport::dca::types::DCAFrequency,
    [Daily, Weekly, Fortnightly, Monthly]
);
impl_java_enum!(
    "com/longport/dca/DCAStatus",
    longport::dca::types::DCAStatus,
    [Active, Suspended, Finished]
);
impl_java_enum!(
    "com/longport/calendar/CalendarCategory",
    longport::calendar::types::CalendarCategory,
    [
        Report, Dividend, Split, Ipo, MacroData, Closed, Meeting, Merge
    ]
);
impl_java_enum!(
    "com/longport/fundamental/FinancialReportKind",
    longport::fundamental::types::FinancialReportKind,
    [IncomeStatement, BalanceSheet, CashFlow, All]
);
impl_java_enum!(
    "com/longport/fundamental/FinancialReportPeriod",
    longport::fundamental::types::FinancialReportPeriod,
    [Annual, SemiAnnual, Q1, Q2, Q3, QuarterlyFull, ThreeQ]
);
impl_java_enum!(
    "com/longport/market/BrokerHoldingPeriod",
    longport::market::types::BrokerHoldingPeriod,
    [Rct1, Rct5, Rct20, Rct60]
);
impl_java_enum!(
    "com/longport/market/AhPremiumPeriod",
    longport::market::types::AhPremiumPeriod,
    [Min1, Min5, Min15, Min30, Min60, Day, Week, Month, Year]
);
impl_java_enum!(
    "com/longport/portfolio/FlowDirection",
    longport::portfolio::types::FlowDirection,
    [Unknown, Buy, Sell]
);
impl_java_enum!(
    "com/longport/portfolio/AssetType",
    longport::portfolio::types::AssetType,
    [Unknown, Stock, Fund, Crypto]
);
impl_java_enum!(
    "com/longport/fundamental/InstitutionRecommend",
    longport::fundamental::types::InstitutionRecommend,
    [
        Unknown,
        StrongBuy,
        Buy,
        Hold,
        Sell,
        StrongSell,
        Underperform,
        NoOpinion
    ]
);
impl_java_enum!(
    "com/longport/agent/ConversationStatus",
    longport::agent::ConversationStatus,
    [Succeeded, Interrupted, Failed, Stopped]
);

// `TriggerPriceType` carries a catch-all `Unknown(i32)` data variant that the
// unit-variant `impl_java_enum!` macro cannot express, so bridge it by hand.
// The Java side is a plain enum, so an unknown wire value maps to `Unknown` and
// back to `Unknown(0)`.
#[allow(non_upper_case_globals)]
static com_longbridge_grid_TriggerPriceType: std::sync::OnceLock<jni::objects::GlobalRef> =
    std::sync::OnceLock::new();

impl ClassLoader for longport::grid::TriggerPriceType {
    fn init(env: &mut JNIEnv) {
        let cls = jni::descriptors::Desc::<jni::objects::JClass>::lookup(
            "com/longport/grid/TriggerPriceType",
            env,
        )
        .expect("com/longport/grid/TriggerPriceType");
        let _ = com_longbridge_grid_TriggerPriceType.set(env.new_global_ref(&*cls).unwrap());
    }

    fn class_ref() -> jni::objects::GlobalRef {
        com_longbridge_grid_TriggerPriceType.get().cloned().unwrap()
    }
}

impl JSignature for longport::grid::TriggerPriceType {
    #[inline]
    fn signature() -> Cow<'static, str> {
        "Lcom/longport/grid/TriggerPriceType;".into()
    }
}

impl FromJValue for longport::grid::TriggerPriceType {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        use longport::grid::TriggerPriceType;

        let cls = <Self as ClassLoader>::class_ref();
        let value = value.l()?;
        if value.is_null() {
            crate::error::throw_illegal_argument(
                env,
                "argument of enum type com/longport/grid/TriggerPriceType must not be null",
            );
            return Err(jni::errors::Error::JavaException);
        }
        for (name, variant) in [
            ("Spread", TriggerPriceType::Spread),
            ("Percent", TriggerPriceType::Percent),
            ("Unknown", TriggerPriceType::Unknown(0)),
        ] {
            let r = env
                .get_static_field(&cls, name, "Lcom/longport/grid/TriggerPriceType;")?
                .l()?;
            if env.is_same_object(&value, r)? {
                return Ok(variant);
            }
        }
        crate::error::throw_illegal_argument(
            env,
            "unrecognized enum value for com/longport/grid/TriggerPriceType",
        );
        Err(jni::errors::Error::JavaException)
    }
}

impl IntoJValue for longport::grid::TriggerPriceType {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        use longport::grid::TriggerPriceType;

        let cls = <Self as ClassLoader>::class_ref();
        let name = match self {
            TriggerPriceType::Spread => "Spread",
            TriggerPriceType::Percent => "Percent",
            TriggerPriceType::Unknown(_) => "Unknown",
        };
        env.get_static_field(&cls, name, "Lcom/longport/grid/TriggerPriceType;")
    }
}

// `GridLimitEvent` carries a catch-all `Unknown(i32)` data variant that the
// unit-variant `impl_java_enum!` macro cannot express, so bridge it by hand.
// The Java side is a plain enum, so an unknown wire value maps to `Unknown` and
// back to `Unknown(0)`.
#[allow(non_upper_case_globals)]
static com_longbridge_grid_GridLimitEvent: std::sync::OnceLock<jni::objects::GlobalRef> =
    std::sync::OnceLock::new();

impl ClassLoader for longport::grid::GridLimitEvent {
    fn init(env: &mut JNIEnv) {
        let cls = jni::descriptors::Desc::<jni::objects::JClass>::lookup(
            "com/longport/grid/GridLimitEvent",
            env,
        )
        .expect("com/longport/grid/GridLimitEvent");
        let _ = com_longbridge_grid_GridLimitEvent.set(env.new_global_ref(&*cls).unwrap());
    }

    fn class_ref() -> jni::objects::GlobalRef {
        com_longbridge_grid_GridLimitEvent.get().cloned().unwrap()
    }
}

impl JSignature for longport::grid::GridLimitEvent {
    #[inline]
    fn signature() -> Cow<'static, str> {
        "Lcom/longport/grid/GridLimitEvent;".into()
    }
}

impl FromJValue for longport::grid::GridLimitEvent {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        use longport::grid::GridLimitEvent;

        let cls = <Self as ClassLoader>::class_ref();
        let value = value.l()?;
        if value.is_null() {
            crate::error::throw_illegal_argument(
                env,
                "argument of enum type com/longport/grid/GridLimitEvent must not be null",
            );
            return Err(jni::errors::Error::JavaException);
        }
        for (name, variant) in [
            ("Ignore", GridLimitEvent::Ignore),
            ("CloseAtLast", GridLimitEvent::CloseAtLast),
            ("Unknown", GridLimitEvent::Unknown(0)),
        ] {
            let r = env
                .get_static_field(&cls, name, "Lcom/longport/grid/GridLimitEvent;")?
                .l()?;
            if env.is_same_object(&value, r)? {
                return Ok(variant);
            }
        }
        crate::error::throw_illegal_argument(
            env,
            "unrecognized enum value for com/longport/grid/GridLimitEvent",
        );
        Err(jni::errors::Error::JavaException)
    }
}

impl IntoJValue for longport::grid::GridLimitEvent {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        use longport::grid::GridLimitEvent;

        let cls = <Self as ClassLoader>::class_ref();
        let name = match self {
            GridLimitEvent::Ignore => "Ignore",
            GridLimitEvent::CloseAtLast => "CloseAtLast",
            GridLimitEvent::Unknown(_) => "Unknown",
        };
        env.get_static_field(&cls, name, "Lcom/longport/grid/GridLimitEvent;")
    }
}

// `GridTimeInForce` carries a catch-all `Unknown(i32)` data variant that the
// unit-variant `impl_java_enum!` macro cannot express, so bridge it by hand.
// The Java side is a plain enum, so an unknown wire value maps to `Unknown` and
// back to `Unknown(0)`.
#[allow(non_upper_case_globals)]
static com_longbridge_grid_GridTimeInForce: std::sync::OnceLock<jni::objects::GlobalRef> =
    std::sync::OnceLock::new();

impl ClassLoader for longport::grid::GridTimeInForce {
    fn init(env: &mut JNIEnv) {
        let cls = jni::descriptors::Desc::<jni::objects::JClass>::lookup(
            "com/longport/grid/GridTimeInForce",
            env,
        )
        .expect("com/longport/grid/GridTimeInForce");
        let _ = com_longbridge_grid_GridTimeInForce.set(env.new_global_ref(&*cls).unwrap());
    }

    fn class_ref() -> jni::objects::GlobalRef {
        com_longbridge_grid_GridTimeInForce.get().cloned().unwrap()
    }
}

impl JSignature for longport::grid::GridTimeInForce {
    #[inline]
    fn signature() -> Cow<'static, str> {
        "Lcom/longport/grid/GridTimeInForce;".into()
    }
}

impl FromJValue for longport::grid::GridTimeInForce {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        use longport::grid::GridTimeInForce;

        let cls = <Self as ClassLoader>::class_ref();
        let value = value.l()?;
        if value.is_null() {
            crate::error::throw_illegal_argument(
                env,
                "argument of enum type com/longport/grid/GridTimeInForce must not be null",
            );
            return Err(jni::errors::Error::JavaException);
        }
        for (name, variant) in [
            ("Day", GridTimeInForce::Day),
            ("GoodTilCanceled", GridTimeInForce::GoodTilCanceled),
            ("GoodTilDate", GridTimeInForce::GoodTilDate),
            ("Unknown", GridTimeInForce::Unknown(0)),
        ] {
            let r = env
                .get_static_field(&cls, name, "Lcom/longport/grid/GridTimeInForce;")?
                .l()?;
            if env.is_same_object(&value, r)? {
                return Ok(variant);
            }
        }
        crate::error::throw_illegal_argument(
            env,
            "unrecognized enum value for com/longport/grid/GridTimeInForce",
        );
        Err(jni::errors::Error::JavaException)
    }
}

impl IntoJValue for longport::grid::GridTimeInForce {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        use longport::grid::GridTimeInForce;

        let cls = <Self as ClassLoader>::class_ref();
        let name = match self {
            GridTimeInForce::Day => "Day",
            GridTimeInForce::GoodTilCanceled => "GoodTilCanceled",
            GridTimeInForce::GoodTilDate => "GoodTilDate",
            GridTimeInForce::Unknown(_) => "Unknown",
        };
        env.get_static_field(&cls, name, "Lcom/longport/grid/GridTimeInForce;")
    }
}
