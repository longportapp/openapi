package com.longport.quote;

import java.math.BigDecimal;

/**
 * Capital distribution by trade size.
 */
public class CapitalDistribution {
    private BigDecimal large;
    private BigDecimal medium;
    private BigDecimal small;

    /**
     * Returns the large-order capital flow.
     *
     * @return the large-order capital flow
     */
    public BigDecimal getLarge() {
        return large;
    }

    /**
     * Returns the medium-order capital flow.
     *
     * @return the medium-order capital flow
     */
    public BigDecimal getMedium() {
        return medium;
    }

    /**
     * Returns the small-order capital flow.
     *
     * @return the small-order capital flow
     */
    public BigDecimal getSmall() {
        return small;
    }

    @Override
    public String toString() {
        return "CapitalDistribution [large=" + large + ", medium=" + medium + ", small=" + small + "]";
    }
}
