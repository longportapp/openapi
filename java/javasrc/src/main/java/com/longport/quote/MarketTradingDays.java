package com.longport.quote;

import java.time.LocalDate;
import java.util.Arrays;

/**
 * Market trading days for a given date range.
 */
public class MarketTradingDays {
    private LocalDate[] tradingDays;
    private LocalDate[] halfTradingDays;

    /**
     * Returns the full trading days.
     *
     * @return the full trading days
     */
    public LocalDate[] getTradingDays() {
        return tradingDays;
    }

    /**
     * Returns the half trading days (e.g. early-close sessions).
     *
     * @return the half trading days
     */
    public LocalDate[] getHalfTradingDays() {
        return halfTradingDays;
    }

    @Override
    public String toString() {
        return "MarketTradingDays [halfTradingDays=" + Arrays.toString(halfTradingDays) + ", tradingDays="
                + Arrays.toString(tradingDays) + "]";
    }
}
