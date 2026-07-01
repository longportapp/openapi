package com.longport.trade;

import java.math.BigDecimal;

/**
 * Response for max purchase quantity estimation
 */
public class EstimateMaxPurchaseQuantityResponse {
    private BigDecimal cashMaxQty;
    private BigDecimal marginMaxQty;

    /**
     * Returns the maximum quantity available with cash.
     *
     * @return maximum cash quantity
     */
    public BigDecimal getCashMaxQty() {
        return cashMaxQty;
    }

    /**
     * Returns the maximum quantity available with margin.
     *
     * @return maximum margin quantity
     */
    public BigDecimal getMarginMaxQty() {
        return marginMaxQty;
    }

    @Override
    public String toString() {
        return "EstimateMaxPurchaseQuantityResponse [cashMaxQty=" + cashMaxQty + ", marginMaxQty=" + marginMaxQty + "]";
    }
}
