package com.longport.quote;

import java.util.Arrays;

/**
 * Active subscription for a security.
 */
public class Subscription {
    private String symbol;
    private int subTypes;
    private Period[] candlesticks;

    /**
     * Returns the subscribed security symbol.
     *
     * @return the subscribed security symbol
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the subscribed data types as a bitfield (see SubFlags).
     *
     * @return the subscribed data types
     */
    public int getSubTypes() {
        return subTypes;
    }

    /**
     * Returns the candlestick periods subscribed for this security.
     *
     * @return the subscribed candlestick periods
     */
    public Period[] getCandlesticks() {
        return candlesticks;
    }

    @Override
    public String toString() {
        return "Subscription [candlesticks=" + Arrays.toString(candlesticks) + ", subTypes=" + subTypes + ", symbol="
                + symbol + "]";
    }
}
