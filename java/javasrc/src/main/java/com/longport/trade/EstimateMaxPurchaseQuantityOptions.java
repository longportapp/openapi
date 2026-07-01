package com.longport.trade;

import java.math.BigDecimal;

/**
 * Options for estimating the maximum purchase quantity
 */
@SuppressWarnings("unused")
public class EstimateMaxPurchaseQuantityOptions {
    private String symbol;
    private OrderType orderType;
    private OrderSide side;
    private BigDecimal price;
    private String currency;
    private String orderId;
    private boolean fractionalShares;

    /**
     * Constructs options for estimating the maximum purchase quantity.
     *
     * @param symbol    security symbol
     * @param orderType order type
     * @param side      order side
     */
    public EstimateMaxPurchaseQuantityOptions(String symbol, OrderType orderType, OrderSide side) {
        this.symbol = symbol;
        this.orderType = orderType;
        this.side = side;
    }

    /**
     * Sets the order price.
     *
     * @param price order price
     * @return this instance for chaining
     */
    public EstimateMaxPurchaseQuantityOptions setPrice(BigDecimal price) {
        this.price = price;
        return this;
    }

    /**
     * Sets the settlement currency.
     *
     * @param currency settlement currency
     * @return this instance for chaining
     */
    public EstimateMaxPurchaseQuantityOptions setCurrency(String currency) {
        this.currency = currency;
        return this;
    }

    /**
     * Sets the order ID (for replacement orders).
     *
     * @param orderId order ID
     * @return this instance for chaining
     */
    public EstimateMaxPurchaseQuantityOptions setOrderId(String orderId) {
        this.orderId = orderId;
        return this;
    }

    /**
     * Enables fractional shares estimation.
     *
     * @return this instance for chaining
     */
    public EstimateMaxPurchaseQuantityOptions fractionalShares() {
        this.fractionalShares = true;
        return this;
    }
}
