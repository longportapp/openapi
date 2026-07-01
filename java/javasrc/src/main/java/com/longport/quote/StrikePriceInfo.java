package com.longport.quote;

import java.math.BigDecimal;

/**
 * Strike price information for an option chain.
 */
public class StrikePriceInfo {
    private BigDecimal price;
    private String callSymbol;
    private String putSymbol;
    private boolean standard;

    /**
     * Returns the strike price.
     *
     * @return the strike price
     */
    public BigDecimal getPrice() {
        return price;
    }

    /**
     * Returns the symbol of the call option at this strike.
     *
     * @return the call option symbol
     */
    public String getCallSymbol() {
        return callSymbol;
    }

    /**
     * Returns the symbol of the put option at this strike.
     *
     * @return the put option symbol
     */
    public String getPutSymbol() {
        return putSymbol;
    }

    /**
     * Returns whether this is a standard strike price.
     *
     * @return {@code true} if this is a standard strike price
     */
    public boolean isStandard() {
        return standard;
    }

    @Override
    public String toString() {
        return "StrikePriceInfo [callSymbol=" + callSymbol + ", price=" + price + ", putSymbol=" + putSymbol
                + ", standard=" + standard + "]";
    }
}
