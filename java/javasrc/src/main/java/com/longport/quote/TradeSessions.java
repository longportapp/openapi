package com.longport.quote;

/**
 * Trade sessions filter for candlestick and intraday queries
 */
public enum TradeSessions {
    /** Intraday session only */
    Intraday,
    /** All sessions (intraday + pre/post/overnight) */
    All,
}
