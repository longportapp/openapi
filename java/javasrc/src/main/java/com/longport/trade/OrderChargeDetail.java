package com.longport.trade;

import java.math.BigDecimal;
import java.util.Arrays;

/**
 * Order charge detail
 */
public class OrderChargeDetail {
    private BigDecimal totalAmount;
    private String currency;
    private OrderChargeItem[] items;

    /**
     * Returns the total charge amount.
     *
     * @return total charge amount
     */
    public BigDecimal getTotalAmount() {
        return totalAmount;
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
     * Returns the charge item categories.
     *
     * @return charge item categories
     */
    public OrderChargeItem[] getItems() {
        return items;
    }

    @Override
    public String toString() {
        return "OrderChargeDetail [totalAmount=" + totalAmount + ", currency=" + currency + ", items="
                + Arrays.toString(items) + "]";
    }
}
