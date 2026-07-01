package com.longport.quote;

import java.util.Arrays;

/**
 * Response for historical market temperature query
 */
public class HistoryMarketTemperatureResponse {
    private Granularity granularity;
    private MarketTemperature[] records;

    /**
     * Returns the granularity of the records.
     *
     * @return granularity
     */
    public Granularity getGranularity() {
        return granularity;
    }

    /**
     * Returns the historical market temperature records.
     *
     * @return records
     */
    public MarketTemperature[] getRecords() {
        return records;
    }

    @Override
    public String toString() {
        return "HistoryMarketTemperatureResponse [granularity=" + granularity + ", records=" + Arrays.toString(records)
                + "]";
    }

}
