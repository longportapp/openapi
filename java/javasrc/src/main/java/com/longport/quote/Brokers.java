package com.longport.quote;

import java.util.Arrays;

/**
 * Brokers at a single price level in the bid/ask queue.
 */
public class Brokers {
    private int position;
    private int[] brokerIds;

    /**
     * Returns the position (1-based) in the bid/ask queue.
     *
     * @return the position
     */
    public int getPosition() {
        return position;
    }

    /**
     * Returns the broker IDs at this position.
     *
     * @return the broker IDs
     */
    public int[] getBrokerIds() {
        return brokerIds;
    }

    @Override
    public String toString() {
        return "Brokers [brokerIds=" + Arrays.toString(brokerIds) + ", position=" + position + "]";
    }
}
