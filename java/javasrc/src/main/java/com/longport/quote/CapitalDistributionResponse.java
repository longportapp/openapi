package com.longport.quote;

import java.time.OffsetDateTime;

/**
 * Capital distribution response.
 */
public class CapitalDistributionResponse {
    private OffsetDateTime timestamp;
    private CapitalDistribution capitalIn;
    private CapitalDistribution capitalOut;

    /**
     * Returns the timestamp of the data.
     *
     * @return the timestamp
     */
    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    /**
     * Returns the inflow capital distribution.
     *
     * @return the inflow capital distribution
     */
    public CapitalDistribution getCapitalIn() {
        return capitalIn;
    }

    /**
     * Returns the outflow capital distribution.
     *
     * @return the outflow capital distribution
     */
    public CapitalDistribution getCapitalOut() {
        return capitalOut;
    }

    @Override
    public String toString() {
        return "CapitalDistributionResponse [capitalIn=" + capitalIn + ", capitalOut=" + capitalOut + ", timestamp="
                + timestamp + "]";
    }
}
