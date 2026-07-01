package com.longport.quote;

import java.util.Arrays;

/**
 * Real-time broker queue push event.
 */
public class PushBrokers {
    private Brokers[] askBrokers;
    private Brokers[] bidBrokers;

    /**
     * Returns the ask-side broker queue.
     *
     * @return the ask-side broker queue
     */
    public Brokers[] getAskBrokers() {
        return askBrokers;
    }

    /**
     * Returns the bid-side broker queue.
     *
     * @return the bid-side broker queue
     */
    public Brokers[] getBidBrokers() {
        return bidBrokers;
    }

    @Override
    public String toString() {
        return "PushBrokers [askBrokers=" + Arrays.toString(askBrokers) + ", bidBrokers=" + Arrays.toString(bidBrokers)
                + "]";
    }
}
