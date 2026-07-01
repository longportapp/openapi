package com.longport.quote;

import java.util.Arrays;

/**
 * Security broker queue (ask and bid sides).
 */
public class SecurityBrokers {
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
        return "SecurityBrokers [askBrokers=" + Arrays.toString(askBrokers) + ", bidBrokers="
                + Arrays.toString(bidBrokers) + "]";
    }
}
