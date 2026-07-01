package com.longport.trade;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * Fund position
 */
public class FundPosition {
    private String symbol;
    private BigDecimal currentNetAssetValue;
    private OffsetDateTime netAssetValueDay;
    private String symbolName;
    private String currency;
    private BigDecimal costNetAssetValue;
    private BigDecimal holdingUnits;

    /**
     * Returns the fund symbol.
     *
     * @return fund symbol
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the current net asset value.
     *
     * @return current net asset value
     */
    public BigDecimal getCurrentNetAssetValue() {
        return currentNetAssetValue;
    }

    /**
     * Returns the date of the net asset value.
     *
     * @return net asset value date
     */
    public OffsetDateTime getNetAssetValueDay() {
        return netAssetValueDay;
    }

    /**
     * Returns the fund name.
     *
     * @return fund name
     */
    public String getSymbolName() {
        return symbolName;
    }

    /**
     * Returns the currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns the cost net asset value.
     *
     * @return cost net asset value
     */
    public BigDecimal getCostNetAssetValue() {
        return costNetAssetValue;
    }

    /**
     * Returns the holding units.
     *
     * @return holding units
     */
    public BigDecimal getHoldingUnits() {
        return holdingUnits;
    }

    @Override
    public String toString() {
        return "FundPosition [symbol=" + symbol + ", currentNetAssetValue=" + currentNetAssetValue
                + ", netAssetValueDay=" + netAssetValueDay + ", symbolName=" + symbolName + ", currency=" + currency
                + ", costNetAssetValue=" + costNetAssetValue + ", holdingUnits=" + holdingUnits + "]";
    }

}
