package com.longport.quote;

/**
 * Warrant sort field
 */
public enum WarrantSortBy {
    /** Latest price */
    LastDone,
    /** Change rate */
    ChangeRate,
    /** Change value */
    ChangeValue,
    /** Volume */
    Volume,
    /** Turnover */
    Turnover,
    /** Expiry date */
    ExpiryDate,
    /** Strike price */
    StrikePrice,
    /** Upper bound price */
    UpperStrikePrice,
    /** Lower bound price */
    LowerStrikePrice,
    /** Outstanding quantity */
    OutstandingQuantity,
    /** Outstanding ratio */
    OutstandingRatio,
    /** Premium */
    Premium,
    /** In/out of the bound */
    ItmOtm,
    /** Implied volatility */
    ImpliedVolatility,
    /** Delta */
    Delta,
    /** Call price */
    CallPrice,
    /** Price interval from the call price */
    ToCallPrice,
    /** Effective leverage */
    EffectiveLeverage,
    /** Leverage ratio */
    LeverageRatio,
    /** Conversion ratio */
    ConversionRatio,
    /** Breakeven point */
    BalancePoint,
    /** Status */
    Status,
}
