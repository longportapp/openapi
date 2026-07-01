package com.longport.trade;

import java.math.BigDecimal;

/**
 * A single fee item in an order charge
 */
public class OrderChargeFee {
    private String code;
    private String name;
    private BigDecimal amount;
    private String currency;

    /**
     * Returns the fee code.
     *
     * @return fee code
     */
    public String getCode() {
        return code;
    }

    /**
     * Returns the fee name.
     *
     * @return fee name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns the fee amount.
     *
     * @return fee amount
     */
    public BigDecimal getAmount() {
        return amount;
    }

    /**
     * Returns the currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    @Override
    public String toString() {
        return "OrderChargeFee [code=" + code + ", name=" + name + ", amount=" + amount + ", currency=" + currency
                + "]";
    }
}
