package com.longport.trade;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * Cash flow record
 */
public class CashFlow {
    private String transactionFlowName;
    private CashFlowDirection direction;
    private BalanceType businessType;
    private BigDecimal balance;
    private String currency;
    private OffsetDateTime businessTime;
    private String symbol;
    private String description;

    /**
     * Returns the transaction flow name.
     *
     * @return transaction flow name
     */
    public String getTransactionFlowName() {
        return transactionFlowName;
    }

    /**
     * Returns the cash flow direction.
     *
     * @return cash flow direction
     */
    public CashFlowDirection getDirection() {
        return direction;
    }

    /**
     * Returns the business type (balance type).
     *
     * @return business type
     */
    public BalanceType getBusinessType() {
        return businessType;
    }

    /**
     * Returns the cash balance.
     *
     * @return balance
     */
    public BigDecimal getBalance() {
        return balance;
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
     * Returns the business time.
     *
     * @return business time
     */
    public OffsetDateTime getBusinessTime() {
        return businessTime;
    }

    /**
     * Returns the associated security symbol.
     *
     * @return security symbol
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the description of the cash flow.
     *
     * @return description
     */
    public String getDescription() {
        return description;
    }

    @Override
    public String toString() {
        return "CashFlow [balance=" + balance + ", businessTime=" + businessTime + ", businessType=" + businessType
                + ", currency=" + currency + ", description=" + description + ", direction=" + direction + ", symbol="
                + symbol + ", transactionFlowName=" + transactionFlowName + "]";
    }
}
