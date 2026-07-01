package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * Intraday line data point.
 */
public class IntradayLine {
    private BigDecimal price;
    private OffsetDateTime timestamp;
    private long volume;
    private BigDecimal turnover;
    private BigDecimal avgPrice;

    /**
     * Returns the price at this data point.
     *
     * @return the price
     */
    public BigDecimal getPrice() {
        return price;
    }

    /**
     * Returns the timestamp of this data point.
     *
     * @return the timestamp
     */
    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    /**
     * Returns the trading volume up to this point.
     *
     * @return the trading volume
     */
    public long getVolume() {
        return volume;
    }

    /**
     * Returns the turnover up to this point.
     *
     * @return the turnover
     */
    public BigDecimal getTurnover() {
        return turnover;
    }

    /**
     * Returns the volume-weighted average price.
     *
     * @return the average price
     */
    public BigDecimal getAvgPrice() {
        return avgPrice;
    }

    @Override
    public String toString() {
        return "IntradayLine [avgPrice=" + avgPrice + ", price=" + price + ", timestamp=" + timestamp + ", turnover="
                + turnover + ", volume=" + volume + "]";
    }
}
