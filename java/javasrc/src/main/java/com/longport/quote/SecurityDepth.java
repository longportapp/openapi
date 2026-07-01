package com.longport.quote;

import java.util.Arrays;

/**
 * Security order book depth (ask and bid sides).
 */
public class SecurityDepth {
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
        return "SecurityDepth [asks=" + Arrays.toString(asks) + ", bids=" + Arrays.toString(bids) + "]";
    }
}
