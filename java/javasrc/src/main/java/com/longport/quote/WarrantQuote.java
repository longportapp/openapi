package com.longport.quote;

import java.math.BigDecimal;
import java.time.LocalDate;
import java.time.OffsetDateTime;

/**
 * Quote of a warrant security.
 */
public class WarrantQuote {
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
    private BigDecimal impliedVolatility;
    private long openInterest;
    private LocalDate expiryDate;
    private LocalDate lastTradeDate;
    private BigDecimal outstandingRatio;
    private long outstandingQuantity;
    private BigDecimal conversionRatio;
    private WarrantType category;
    private BigDecimal strikePrice;
    private BigDecimal upperStrikePrice;
    private BigDecimal lowerStrikePrice;
    private BigDecimal callPrice;
    private String underlyingSymbol;

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
     * Returns the security trading status.
     *
     * @return the security trading status
     */
    public TradeStatus getTradeStatus() {
        return tradeStatus;
    }

    /**
     * Returns the implied volatility.
     *
     * @return the implied volatility
     */
    public BigDecimal getImpliedVolatility() {
        return impliedVolatility;
    }

    /**
     * Returns the open interest.
     *
     * @return the open interest
     */
    public long getOpenInterest() {
        return openInterest;
    }

    /**
     * Returns the expiry date.
     *
     * @return the expiry date
     */
    public LocalDate getExpiryDate() {
        return expiryDate;
    }

    /**
     * Returns the last tradable date.
     *
     * @return the last tradable date
     */
    public LocalDate getLastTradeDate() {
        return lastTradeDate;
    }

    /**
     * Returns the outstanding ratio.
     *
     * @return the outstanding ratio
     */
    public BigDecimal getOutstandingRatio() {
        return outstandingRatio;
    }

    /**
     * Returns the outstanding quantity.
     *
     * @return the outstanding quantity
     */
    public long getOutstandingQuantity() {
        return outstandingQuantity;
    }

    /**
     * Returns the conversion ratio.
     *
     * @return the conversion ratio
     */
    public BigDecimal getConversionRatio() {
        return conversionRatio;
    }

    /**
     * Returns the warrant category (type).
     *
     * @return the warrant category
     */
    public WarrantType getCategory() {
        return category;
    }

    /**
     * Returns the strike price.
     *
     * @return the strike price
     */
    public BigDecimal getStrikePrice() {
        return strikePrice;
    }

    /**
     * Returns the upper bound price (for inline warrants).
     *
     * @return the upper bound price
     */
    public BigDecimal getUpperStrikePrice() {
        return upperStrikePrice;
    }

    /**
     * Returns the lower bound price (for inline warrants).
     *
     * @return the lower bound price
     */
    public BigDecimal getLowerStrikePrice() {
        return lowerStrikePrice;
    }

    /**
     * Returns the call price.
     *
     * @return the call price
     */
    public BigDecimal getCallPrice() {
        return callPrice;
    }

    /**
     * Returns the underlying security symbol.
     *
     * @return the underlying security symbol
     */
    public String getUnderlyingSymbol() {
        return underlyingSymbol;
    }

    @Override
    public String toString() {
        return "WarrantQuote [callPrice=" + callPrice + ", category=" + category + ", conversionRatio="
                + conversionRatio + ", expiryDate=" + expiryDate + ", high=" + high + ", impliedVolatility="
                + impliedVolatility + ", lastDone=" + lastDone + ", lastTradeDate=" + lastTradeDate + ", low=" + low
                + ", lowerStrikePrice=" + lowerStrikePrice + ", open=" + open + ", openInterest=" + openInterest
                + ", outstandingQuantity=" + outstandingQuantity + ", outstandingRatio=" + outstandingRatio
                + ", prevClose=" + prevClose + ", strikePrice=" + strikePrice + ", symbol=" + symbol + ", timestamp="
                + timestamp + ", tradeStatus=" + tradeStatus + ", turnover=" + turnover + ", underlyingSymbol="
                + underlyingSymbol + ", upperStrikePrice=" + upperStrikePrice + ", volume=" + volume + "]";
    }
}
