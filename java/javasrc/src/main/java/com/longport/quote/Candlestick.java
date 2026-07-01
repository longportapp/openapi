package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * Candlestick (OHLCV bar).
 */
public class Candlestick {
    private BigDecimal close;
    private BigDecimal open;
    private BigDecimal low;
    private BigDecimal high;
    private long volume;
    private BigDecimal turnover;
    private OffsetDateTime timestamp;
    private TradeSession tradeSession;

    /**
     * Returns the closing price.
     *
     * @return the closing price
     */
    public BigDecimal getClose() {
        return close;
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
     * Returns the lowest price.
     *
     * @return the lowest price
     */
    public BigDecimal getLow() {
        return low;
    }

    /**
     * Returns the highest price.
     *
     * @return the highest price
     */
    public BigDecimal getHigh() {
        return high;
    }

    /**
     * Returns the trading volume.
     *
     * @return the trading volume
     */
    public long getVolume() {
        return volume;
    }

    /**
     * Returns the turnover.
     *
     * @return the turnover
     */
    public BigDecimal getTurnover() {
        return turnover;
    }

    /**
     * Returns the timestamp of this candlestick.
     *
     * @return the timestamp
     */
    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    /**
     * Returns the trade session this candlestick belongs to.
     *
     * @return the trade session
     */
    public TradeSession getTradeSession() {
        return tradeSession;
    }

    @Override
    public String toString() {
        return "Candlestick [close=" + close + ", open=" + open + ", low=" + low + ", high=" + high + ", volume="
                + volume + ", turnover=" + turnover + ", timestamp=" + timestamp + ", tradeSession=" + tradeSession
                + "]";
    }

}
