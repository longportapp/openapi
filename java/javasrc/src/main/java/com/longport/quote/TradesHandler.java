package com.longport.quote;

/**
 * Callback interface for real-time trades push events
 */
public interface TradesHandler {
    /**
     * Called when trade updates are received for the subscribed symbol.
     *
     * @param symbol security symbol
     * @param event  trades update event
     */
    void onTrades(String symbol, PushTrades event);
}
