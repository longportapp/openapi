package com.longport.quote;

import java.time.OffsetDateTime;

/**
 * Market temperature data point.
 */
public class MarketTemperature {
    private int temperature;
    private String description;
    private int valuation;
    private int sentiment;
    private OffsetDateTime timestamp;

    /**
     * Returns the market temperature value (0–100).
     *
     * @return the market temperature value
     */
    public int getTemperature() {
        return temperature;
    }

    /**
     * Returns the human-readable temperature description.
     *
     * @return the temperature description
     */
    public String getDescription() {
        return description;
    }

    /**
     * Returns the valuation index.
     *
     * @return the valuation index
     */
    public int getValuation() {
        return valuation;
    }

    /**
     * Returns the sentiment index.
     *
     * @return the sentiment index
     */
    public int getSentiment() {
        return sentiment;
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
        return "MarketTemperature [temperature=" + temperature + ", description=" + description + ", valuation="
                + valuation + ", sentiment=" + sentiment + ", timestamp=" + timestamp + "]";
    }
}