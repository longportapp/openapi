package com.longport.quote;

/**
 * Real-time candlestick push event.
 */
public class PushCandlestick {
    private Period period;
    private Candlestick candlestick;
    private boolean isConfirmed;

    /**
     * Returns the candlestick period.
     *
     * @return the candlestick period
     */
    public Period getPeriod() {
        return period;
    }

    /**
     * Returns the candlestick data.
     *
     * @return the candlestick data
     */
    public Candlestick getCandlestick() {
        return candlestick;
    }

    /**
     * Returns whether this candlestick is confirmed (bar closed).
     *
     * @return {@code true} if this candlestick is confirmed
     */
    public boolean isConfirmed() {
        return isConfirmed;
    }

    @Override
    public String toString() {
        return "PushCandlestick [period=" + period + ", candlestick=" + candlestick
                + ", isConfirmed=" + isConfirmed + "]";
    }

}
