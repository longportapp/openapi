package com.longport.trade;

import java.util.Arrays;

/**
 * Response containing all fund positions
 */
public class FundPositionsResponse {
    private FundPositionChannel[] channels;

    /**
     * Returns the fund position channels.
     *
     * @return fund position channels
     */
    public FundPositionChannel[] getChannels() {
        return channels;
    }

    @Override
    public String toString() {
        return "FundPositionsResponse [channels=" + Arrays.toString(channels) + "]";
    }
}
