package com.longport.quote;

/**
 * Callback interface for real-time broker queue push events
 */
public interface BrokersHandler {
    /**
     * Called when a broker queue update is received for the subscribed symbol.
     *
     * @param symbol security symbol
     * @param event  broker queue update event
     */
    void onBrokers(String symbol, PushBrokers event);
}
