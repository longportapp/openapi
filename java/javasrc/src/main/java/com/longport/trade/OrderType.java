package com.longport.trade;

/**
 * Order type
 */
public enum OrderType {
    /** Unknown */
    Unknown,
    /** Limit order */
    LO,
    /** Enhanced limit order */
    ELO,
    /** Market order */
    MO,
    /** At-auction order */
    AO,
    /** At-auction limit order */
    ALO,
    /** Odd lots order */
    ODD,
    /** Limit if touched */
    LIT,
    /** Market if touched */
    MIT,
    /** Trailing limit if touched (amount) */
    TSLPAMT,
    /** Trailing limit if touched (percentage) */
    TSLPPCT,
    /** Trailing market if touched (amount) */
    TSMAMT,
    /** Trailing market if touched (percentage) */
    TSMPCT,
    /** Special limit order */
    SLO,
}
