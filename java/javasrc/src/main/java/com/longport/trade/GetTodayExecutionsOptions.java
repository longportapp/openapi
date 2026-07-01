package com.longport.trade;

/**
 * Options for querying today's executions
 */
@SuppressWarnings("unused")
public class GetTodayExecutionsOptions {
    private String symbol;
    private String orderId;

    /**
     * Filters by security symbol.
     *
     * @param symbol security symbol
     * @return this instance for chaining
     */
    public GetTodayExecutionsOptions setSymbol(String symbol) {
        this.symbol = symbol;
        return this;
    }

    /**
     * Filters by order ID.
     *
     * @param orderId order ID
     * @return this instance for chaining
     */
    public GetTodayExecutionsOptions setOrderId(String orderId) {
        this.orderId = orderId;
        return this;
    }
}
