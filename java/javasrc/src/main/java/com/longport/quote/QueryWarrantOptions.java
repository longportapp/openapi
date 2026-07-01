package com.longport.quote;

/**
 * Query options for warrant list search
 */
@SuppressWarnings("unused")
public class QueryWarrantOptions {
    private String symbol;
    private WarrantSortBy sortBy;
    private SortOrderType sortType;
    private WarrantType[] warrantType;
    private int[] issuer;
    private FilterWarrantExpiryDate[] expiryDate;
    private FilterWarrantInOutBoundsType[] priceType;
    private WarrantStatus[] status;

    /**
     * Constructs warrant query options.
     *
     * @param symbol   underlying security symbol
     * @param sortBy   sort field
     * @param sortType sort order
     */
    public QueryWarrantOptions(String symbol, WarrantSortBy sortBy, SortOrderType sortType) {
        this.symbol = symbol;
        this.sortBy = sortBy;
        this.sortType = sortType;
    }

    /**
     * Filters by warrant type.
     *
     * @param warrantType warrant types
     * @return this instance for chaining
     */
    public QueryWarrantOptions setWarrantType(WarrantType[] warrantType) {
        this.warrantType = warrantType;
        return this;
    }

    /**
     * Filters by issuer ID.
     *
     * @param issuer issuer IDs
     * @return this instance for chaining
     */
    public QueryWarrantOptions setIssuer(int[] issuer) {
        this.issuer = issuer;
        return this;
    }

    /**
     * Filters by expiry date range.
     *
     * @param expiryDate expiry date filter
     * @return this instance for chaining
     */
    public QueryWarrantOptions setExpiryDate(FilterWarrantExpiryDate[] expiryDate) {
        this.expiryDate = expiryDate;
        return this;
    }

    /**
     * Filters by in/out of bounds type.
     *
     * @param priceType price type filter
     * @return this instance for chaining
     */
    public QueryWarrantOptions setPriceType(FilterWarrantInOutBoundsType[] priceType) {
        this.priceType = priceType;
        return this;
    }

    /**
     * Filters by warrant status.
     *
     * @param status warrant status filter
     * @return this instance for chaining
     */
    public QueryWarrantOptions setStatus(WarrantStatus[] status) {
        this.status = status;
        return this;
    }

}
