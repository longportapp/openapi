package com.longport.quote;

/**
 * Callback interface for real-time candlestick push events
 */
public interface CandlestickHandler {
    /**
     * Called when a candlestick update is received for the subscribed symbol.
     *
     * @param symbol security symbol
     * @param event  candlestick update event
     */
    void onCandlestick(String symbol, PushCandlestick event);
}
