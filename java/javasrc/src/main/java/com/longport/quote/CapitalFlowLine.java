package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * Capital flow data point for intraday capital flow.
 */
public class CapitalFlowLine {
    private BigDecimal inflow;
    private OffsetDateTime timestamp;

    /**
     * Returns the net inflow amount.
     *
     * @return the net inflow amount
     */
    public BigDecimal getInflow() {
        return inflow;
    }

    /**
     * Returns the timestamp of this data point.
     *
     * @return the timestamp
     */
    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    @Override
    public String toString() {
        return "CapitalFlowLine [inflow=" + inflow + ", timestamp=" + timestamp + "]";
    }
}
