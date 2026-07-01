package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * Real-time quote retrieved from the local subscription cache.
 */
public class RealtimeQuote {
    private String symbol;
    private BigDecimal lastDone;
    private BigDecimal open;
    private BigDecimal high;
    private BigDecimal low;
    private OffsetDateTime timestamp;
    private long volume;
    private BigDecimal turnover;
    private TradeStatus tradeStatus;

    /**
     * Returns the security code.
     *
     * @return the security code
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the latest price.
     *
     * @return the latest price
     */
    public BigDecimal getLastDone() {
        return lastDone;
    }

    /**
     * Returns the opening price.
     *
     * @return the opening price
     */
    public BigDecimal getOpen() {
        return open;
    }

    /**
     * Returns the highest price of the day.
     *
     * @return the highest price of the day
     */
    public BigDecimal getHigh() {
        return high;
    }

    /**
     * Returns the lowest price of the day.
     *
     * @return the lowest price of the day
     */
    public BigDecimal getLow() {
        return low;
    }

    /**
     * Returns the timestamp of the latest price.
     *
     * @return the timestamp of the latest price
     */
    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    /**
     * Returns the cumulative trading volume.
     *
     * @return the cumulative trading volume
     */
    public long getVolume() {
        return volume;
    }

    /**
     * Returns the cumulative turnover.
     *
     * @return the cumulative turnover
     */
    public BigDecimal getTurnover() {
        return turnover;
    }

    /**
     * Returns the security trading status.
     *
     * @return the security trading status
     */
    public TradeStatus getTradeStatus() {
        return tradeStatus;
    }

    @Override
    public String toString() {
        return "RealtimeQuote [high=" + high + ", lastDone=" + lastDone + ", low=" + low + ", open=" + open
                + ", symbol=" + symbol + ", timestamp=" + timestamp + ", tradeStatus=" + tradeStatus + ", turnover="
                + turnover + ", volume=" + volume + "]";
    }
}
