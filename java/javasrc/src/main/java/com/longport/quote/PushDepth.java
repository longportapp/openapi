package com.longport.quote;

import java.util.Arrays;

/**
 * Real-time order book depth push event.
 */
public class PushDepth {
    private Depth[] asks;
    private Depth[] bids;

    /**
     * Returns the ask-side depth levels.
     *
     * @return the ask-side depth levels
     */
    public Depth[] getAsks() {
        return asks;
    }

    /**
     * Returns the bid-side depth levels.
     *
     * @return the bid-side depth levels
     */
    public Depth[] getBids() {
        return bids;
    }

    @Override
    public String toString() {
        return "PushDepth [asks=" + Arrays.toString(asks) + ", bids=" + Arrays.toString(bids) + "]";
    }
}
