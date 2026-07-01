package com.longport.quote;

/**
 * Callback interface for real-time quote push events
 */
public interface QuoteHandler {
    /**
     * Called when a quote update is received for the subscribed symbol.
     *
     * @param symbol security symbol
     * @param event  quote update event
     */
    void onQuote(String symbol, PushQuote event);
}
