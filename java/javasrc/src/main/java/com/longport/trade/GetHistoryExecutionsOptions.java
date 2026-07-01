package com.longport.trade;

import java.time.OffsetDateTime;

/**
 * Options for querying history executions
 */
@SuppressWarnings("unused")
public class GetHistoryExecutionsOptions {
    private String symbol;
    private OffsetDateTime startAt;
    private OffsetDateTime endAt;

    /**
     * Filters by security symbol.
     *
     * @param symbol security symbol
     * @return this instance for chaining
     */
    public GetHistoryExecutionsOptions setSymbol(String symbol) {
        this.symbol = symbol;
        return this;
    }

    /**
     * Sets the start of the query time range.
     *
     * @param startAt start time
     * @return this instance for chaining
     */
    public GetHistoryExecutionsOptions setStartAt(OffsetDateTime startAt) {
        this.startAt = startAt;
        return this;
    }

    /**
     * Sets the end of the query time range.
     *
     * @param endAt end time
     * @return this instance for chaining
     */
    public GetHistoryExecutionsOptions setEndAt(OffsetDateTime endAt) {
        this.endAt = endAt;
        return this;
    }
}
