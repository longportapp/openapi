package com.longport.trade;

/**
 * Order time-in-force type
 */
public enum TimeInForceType {
    /** Unknown */
    Unknown,
    /** Day order */
    Day,
    /** Good till canceled */
    GoodTilCanceled,
    /** Good till date */
    GoodTilDate,
}
