package com.longport.trade;

import java.util.Arrays;

/**
 * A charge item category in an order charge detail
 */
public class OrderChargeItem {
    private ChargeCategoryCode code;
    private String name;
    private OrderChargeFee[] fees;

    /**
     * Returns the charge category code.
     *
     * @return charge category code
     */
    public ChargeCategoryCode getCode() {
        return code;
    }

    /**
     * Returns the charge category name.
     *
     * @return charge category name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns the individual fee items in this category.
     *
     * @return fee items
     */
    public OrderChargeFee[] getFees() {
        return fees;
    }

    @Override
    public String toString() {
        return "OrderChargeItem [code=" + code + ", name=" + name + ", fees=" + Arrays.toString(fees) + "]";
    }
}
