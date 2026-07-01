package com.longport.trade;

/**
 * Options for querying fund positions
 */
@SuppressWarnings("unused")
public class GetFundPositionsOptions {
    private String[] symbols;

    /**
     * Filters by fund symbols.
     *
     * @param symbols fund symbols
     * @return this instance for chaining
     */
    public GetFundPositionsOptions setSymbols(String[] symbols) {
        this.symbols = symbols;
        return this;
    }
}
