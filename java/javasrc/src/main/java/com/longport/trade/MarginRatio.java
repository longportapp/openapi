package com.longport.trade;

import java.math.BigDecimal;

/**
 * Margin ratio information
 */
public class MarginRatio {
    private BigDecimal imFactor;
    private BigDecimal mmFactor;
    private BigDecimal fmFactor;

    /**
     * Returns the initial margin factor.
     *
     * @return initial margin factor
     */
    public BigDecimal getImFactor() {
        return imFactor;
    }

    /**
     * Returns the maintenance margin factor.
     *
     * @return maintenance margin factor
     */
    public BigDecimal getMmFactor() {
        return mmFactor;
    }

    /**
     * Returns the forced-liquidation margin factor.
     *
     * @return forced-liquidation margin factor
     */
    public BigDecimal getFmFactor() {
        return fmFactor;
    }

    @Override
    public String toString() {
        return "MarginRatio [fmFactor=" + fmFactor + ", imFactor=" + imFactor + ", mmFactor=" + mmFactor + "]";
    }

}
