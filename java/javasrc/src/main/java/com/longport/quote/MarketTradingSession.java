package com.longport.quote;

import java.util.Arrays;

import com.longport.Market;

/**
 * Trading sessions for a single market on a given day.
 */
public class MarketTradingSession {
    private Market market;
    private TradingSessionInfo[] tradeSessions;

    /**
     * Returns the market.
     *
     * @return the market
     */
    public Market getMarket() {
        return market;
    }

    /**
     * Returns the trading session time ranges for this market.
     *
     * @return the trading session time ranges
     */
    public TradingSessionInfo[] getTradeSessions() {
        return tradeSessions;
    }

    @Override
    public String toString() {
        return "MarketTradingSession [market=" + market + ", tradeSession=" + Arrays.toString(tradeSessions) + "]";
    }
}
