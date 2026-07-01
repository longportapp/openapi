package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * Quote of a security.
 */
public class SecurityQuote {
    private String symbol;
    private BigDecimal lastDone;
    private BigDecimal prevClose;
    private BigDecimal open;
    private BigDecimal high;
    private BigDecimal low;
    private OffsetDateTime timestamp;
    private long volume;
    private BigDecimal turnover;
    private TradeStatus tradeStatus;
    private PrePostQuote preMarketQuote;
    private PrePostQuote postMarketQuote;
    private PrePostQuote overnightQuote;

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
     * Returns yesterday's closing price.
     *
     * @return yesterday's closing price
     */
    public BigDecimal getPrevClose() {
        return prevClose;
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

    /**
     * Returns the pre-market quote (US stocks only), or null if not available.
     *
     * @return the pre-market quote, or null if not available
     */
    public PrePostQuote getPreMarketQuote() {
        return preMarketQuote;
    }

    /**
     * Returns the post-market quote (US stocks only), or null if not available.
     *
     * @return the post-market quote, or null if not available
     */
    public PrePostQuote getPostMarketQuote() {
        return postMarketQuote;
    }

    /**
     * Returns the overnight quote (US stocks only), or null if not available.
     *
     * @return the overnight quote, or null if not available
     */
    public PrePostQuote getOvernightQuote() {
        return overnightQuote;
    }

    @Override
    public String toString() {
        return "SecurityQuote [symbol=" + symbol + ", lastDone=" + lastDone + ", prevClose=" + prevClose + ", open="
                + open + ", high=" + high + ", low=" + low + ", timestamp=" + timestamp + ", volume=" + volume
                + ", turnover=" + turnover + ", tradeStatus=" + tradeStatus + ", preMarketQuote=" + preMarketQuote
                + ", postMarketQuote=" + postMarketQuote + ", overnightQuote=" + overnightQuote + "]";
    }

}