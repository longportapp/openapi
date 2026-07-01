package com.longport.trade;

import java.time.OffsetDateTime;

/**
 * Options for querying cash flow records
 */
@SuppressWarnings("unused")
public class GetCashFlowOptions {
    private OffsetDateTime startAt;
    private OffsetDateTime endAt;
    private BalanceType businessType;
    private String symbol;
    private int page;
    private int size;

    /**
     * Constructs cash flow query options.
     *
     * @param startAt start of the query time range
     * @param endAt   end of the query time range
     */
    public GetCashFlowOptions(OffsetDateTime startAt, OffsetDateTime endAt) {
        this.startAt = startAt;
        this.endAt = endAt;
    }

    /**
     * Returns the business type filter.
     *
     * @return business type
     */
    public BalanceType getBusinessType() {
        return businessType;
    }

    /**
     * Filters by business type (balance type).
     *
     * @param businessType balance type filter
     * @return this instance for chaining
     */
    public GetCashFlowOptions setBusinessType(BalanceType businessType) {
        this.businessType = businessType;
        return this;
    }

    /**
     * Filters by security symbol.
     *
     * @param symbol security symbol
     * @return this instance for chaining
     */
    public GetCashFlowOptions setSymbol(String symbol) {
        this.symbol = symbol;
        return this;
    }

    /**
     * Sets the page number for pagination.
     *
     * @param page page number
     * @return this instance for chaining
     */
    public GetCashFlowOptions setPage(int page) {
        this.page = page;
        return this;
    }

    /**
     * Sets the page size for pagination.
     *
     * @param size page size
     * @return this instance for chaining
     */
    public GetCashFlowOptions setSize(int size) {
        this.size = size;
        return this;
    }

}
