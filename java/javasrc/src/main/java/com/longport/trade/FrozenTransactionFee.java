package com.longport.trade;

import java.math.BigDecimal;

/**
 * Frozen transaction fee for a single currency
 */
class FrozenTransactionFee {
    private String currency;
    private BigDecimal frozenTransactionFee;

    /**
     * Returns the currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns the frozen transaction fee amount.
     *
     * @return frozen transaction fee amount
     */
    public BigDecimal getFrozenTransactionFee() {
        return frozenTransactionFee;
    }

    @Override
    public String toString() {
        return "FrozenTransactionFee [currency=" + currency + ", frozenTransactionFee=" + frozenTransactionFee + "]";
    }
}