package com.longport.trade;

import java.math.BigDecimal;
import java.util.Arrays;

/**
 * Account balance information
 */
public class AccountBalance {
    private BigDecimal totalCash;
    private BigDecimal maxFinanceAmount;
    private BigDecimal remainingFinanceAmount;
    private int riskLevel;
    private BigDecimal marginCall;
    private String currency;
    private CashInfo[] cashInfos;
    private BigDecimal netAssets;
    private BigDecimal initMargin;
    private BigDecimal maintenanceMargin;
    private BigDecimal buyPower;
    private FrozenTransactionFee[] frozenTransactionFees;

    /**
     * Returns the total cash.
     *
     * @return total cash
     */
    public BigDecimal getTotalCash() {
        return totalCash;
    }

    /**
     * Returns the maximum financing amount.
     *
     * @return maximum financing amount
     */
    public BigDecimal getMaxFinanceAmount() {
        return maxFinanceAmount;
    }

    /**
     * Returns the remaining financing amount.
     *
     * @return remaining financing amount
     */
    public BigDecimal getRemainingFinanceAmount() {
        return remainingFinanceAmount;
    }

    /**
     * Returns the risk level (0-5).
     *
     * @return risk level
     */
    public int getRiskLevel() {
        return riskLevel;
    }

    /**
     * Returns the margin call amount.
     *
     * @return margin call amount
     */
    public BigDecimal getMarginCall() {
        return marginCall;
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
     * Returns the cash details.
     *
     * @return cash details
     */
    public CashInfo[] getCashInfos() {
        return cashInfos;
    }

    /**
     * Returns the net asset value.
     *
     * @return net asset value
     */
    public BigDecimal getNetAssets() {
        return netAssets;
    }

    /**
     * Returns the initial margin.
     *
     * @return initial margin
     */
    public BigDecimal getInitMargin() {
        return initMargin;
    }

    /**
     * Returns the maintenance margin.
     *
     * @return maintenance margin
     */
    public BigDecimal getMaintenanceMargin() {
        return maintenanceMargin;
    }

    /**
     * Returns the buying power.
     *
     * @return buying power
     */
    public BigDecimal getBuyPower() {
        return buyPower;
    }

    /**
     * Returns the frozen transaction fees.
     *
     * @return frozen transaction fees
     */
    public FrozenTransactionFee[] getFrozenTransactionFees() {
        return frozenTransactionFees;
    }

    @Override
    public String toString() {
        return "AccountBalance [totalCash=" + totalCash + ", maxFinanceAmount=" + maxFinanceAmount
                + ", remainingFinanceAmount=" + remainingFinanceAmount + ", riskLevel=" + riskLevel + ", marginCall="
                + marginCall + ", currency=" + currency + ", cashInfos=" + Arrays.toString(cashInfos) + ", netAssets="
                + netAssets + ", initMargin=" + initMargin + ", maintenanceMargin=" + maintenanceMargin + ", buyPower="
                + buyPower + ", frozenTransactionFees=" + Arrays.toString(frozenTransactionFees) + "]";
    }
}
