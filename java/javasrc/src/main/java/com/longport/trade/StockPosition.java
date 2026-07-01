package com.longport.trade;

import java.math.BigDecimal;

import com.longport.Market;

/**
 * Stock position
 */
public class StockPosition {
    private String symbol;
    private String symbolName;
    private BigDecimal quantity;
    private BigDecimal availableQuantity;
    private String currency;
    private BigDecimal costPrice;
    private Market market;
    private BigDecimal initQuantity;

    /**
     * Returns the security symbol.
     *
     * @return security symbol
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the security name.
     *
     * @return security name
     */
    public String getSymbolName() {
        return symbolName;
    }

    /**
     * Returns the holding quantity.
     *
     * @return holding quantity
     */
    public BigDecimal getQuantity() {
        return quantity;
    }

    /**
     * Returns the available (sellable) quantity.
     *
     * @return available quantity
     */
    public BigDecimal getAvailableQuantity() {
        return availableQuantity;
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
     * Returns the cost price.
     *
     * @return cost price
     */
    public BigDecimal getCostPrice() {
        return costPrice;
    }

    /**
     * Returns the market.
     *
     * @return market
     */
    public Market getMarket() {
        return market;
    }

    /**
     * Returns the initial holding quantity at the start of the day.
     *
     * @return initial quantity
     */
    public BigDecimal getInitQuantity() {
        return initQuantity;
    }

    @Override
    public String toString() {
        return "StockPosition [symbol=" + symbol + ", symbolName=" + symbolName + ", quantity=" + quantity
                + ", availableQuantity=" + availableQuantity + ", currency=" + currency + ", costPrice=" + costPrice
                + ", market=" + market + ", initQuantity=" + initQuantity + "]";
    }
}
