package com.longport.trade;

import java.util.Arrays;

/**
 * Response containing all stock positions
 */
public class StockPositionsResponse {
    private StockPositionChannel[] channels;

    /**
     * Returns the stock position channels.
     *
     * @return stock position channels
     */
    public StockPositionChannel[] getChannels() {
        return channels;
    }

    @Override
    public String toString() {
        return "StockPositionsResponse [channels=" + Arrays.toString(channels) + "]";
    }
}
