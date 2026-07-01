package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * Quote of US pre/post market.
 */
public class PrePostQuote {
    private BigDecimal lastDone;
    private OffsetDateTime timestamp;
    private long volume;
    private BigDecimal turnover;
    private BigDecimal high;
    private BigDecimal low;
    private BigDecimal prevClose;

    /**
     * Returns the latest price.
     *
     * @return the latest price
     */
    public BigDecimal getLastDone() {
        return lastDone;
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
     * Returns the highest price.
     *
     * @return the highest price
     */
    public BigDecimal getHigh() {
        return high;
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
     * Returns the close price of the last regular trading session.
     *
     * @return the close price of the last regular trading session
     */
    public BigDecimal getPrevClose() {
        return prevClose;
    }

    @Override
    public String toString() {
        return "PrePostQuote [high=" + high + ", lastDone=" + lastDone + ", low=" + low + ", prevClose=" + prevClose
                + ", timestamp=" + timestamp + ", turnover=" + turnover + ", volume=" + volume + "]";
    }
}
