package com.longport.quote;

import java.util.Arrays;

/**
 * Real-time trades push event.
 */
public class PushTrades {
    private Trade[] trades;

    /**
     * Returns the trades included in this push event.
     *
     * @return the trades
     */
    public Trade[] getTrades() {
        return trades;
    }

    @Override
    public String toString() {
        return "PushTrades [trades=" + Arrays.toString(trades) + "]";
    }
}
