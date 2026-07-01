package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;
import java.time.LocalDate;

/**
 * Quote of an option security.
 */
public class OptionQuote {
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
    private BigDecimal strikePrice;
    private BigDecimal contractMultiplier;
    private OptionType contractType;
    private BigDecimal contractSize;
    private OptionDirection direction;
    private BigDecimal historicalVolatility;
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
     * Returns the number of open positions.
     *
     * @return the number of open positions
     */
    public long getOpenInterest() {
        return openInterest;
    }

    /**
     * Returns the option expiry date.
     *
     * @return the option expiry date
     */
    public LocalDate getExpiryDate() {
        return expiryDate;
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
     * Returns the contract multiplier.
     *
     * @return the contract multiplier
     */
    public BigDecimal getContractMultiplier() {
        return contractMultiplier;
    }

    /**
     * Returns the option type (American / European).
     *
     * @return the option type
     */
    public OptionType getContractType() {
        return contractType;
    }

    /**
     * Returns the contract size.
     *
     * @return the contract size
     */
    public BigDecimal getContractSize() {
        return contractSize;
    }

    /**
     * Returns the option direction (Put / Call).
     *
     * @return the option direction
     */
    public OptionDirection getDirection() {
        return direction;
    }

    /**
     * Returns the underlying security's historical volatility.
     *
     * @return the historical volatility
     */
    public BigDecimal getHistoricalVolatility() {
        return historicalVolatility;
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
        return "OptionQuote [contractMultiplier=" + contractMultiplier + ", contractSize=" + contractSize
                + ", contractType=" + contractType + ", direction=" + direction + ", expiryDate=" + expiryDate
                + ", high=" + high + ", historicalVolatility=" + historicalVolatility + ", impliedVolatility="
                + impliedVolatility + ", lastDone=" + lastDone + ", low=" + low + ", open=" + open + ", openInterest="
                + openInterest + ", prevClose=" + prevClose + ", strikePrice=" + strikePrice + ", symbol=" + symbol
                + ", timestamp=" + timestamp + ", tradeStatus=" + tradeStatus + ", turnover=" + turnover
                + ", underlyingSymbol=" + underlyingSymbol + ", volume=" + volume + "]";
    }
}