package com.longport.trade;

/**
 * Options for querying stock positions
 */
@SuppressWarnings("unused")
public class GetStockPositionsOptions {
    private String[] symbols;

    /**
     * Filters by stock symbols.
     *
     * @param symbols stock symbols
     * @return this instance for chaining
     */
    public GetStockPositionsOptions setSymbols(String[] symbols) {
        this.symbols = symbols;
        return this;
    }
}
