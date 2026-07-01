package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * Real-time quote push event.
 */
public class PushQuote {
    private BigDecimal lastDone;
    private BigDecimal open;
    private BigDecimal high;
    private BigDecimal low;
    private OffsetDateTime timestamp;
    private long volume;
    private BigDecimal turnover;
    private TradeStatus tradeStatus;
    private TradeSession tradeSession;
    private long currentVolume;
    private BigDecimal currentTurnover;

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
     * Returns the cumulative trading volume for the day.
     *
     * @return the cumulative trading volume for the day
     */
    public long getVolume() {
        return volume;
    }

    /**
     * Returns the cumulative turnover for the day.
     *
     * @return the cumulative turnover for the day
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

    /**
     * Returns the trade session that generated this quote.
     *
     * @return the trade session
     */
    public TradeSession getTradeSession() {
        return tradeSession;
    }

    /**
     * Returns the volume of the trade that triggered this push.
     *
     * @return the current trade volume
     */
    public long getCurrentVolume() {
        return currentVolume;
    }

    /**
     * Returns the turnover of the trade that triggered this push.
     *
     * @return the current trade turnover
     */
    public BigDecimal getCurrentTurnover() {
        return currentTurnover;
    }

    @Override
    public String toString() {
        return "PushQuote [lastDone=" + lastDone + ", open=" + open + ", high=" + high + ", low=" + low + ", timestamp="
                + timestamp + ", volume=" + volume + ", turnover=" + turnover + ", tradeStatus=" + tradeStatus
                + ", tradeSession=" + tradeSession + ", currentVolume=" + currentVolume + ", currentTurnover="
                + currentTurnover + "]";
    }

}
