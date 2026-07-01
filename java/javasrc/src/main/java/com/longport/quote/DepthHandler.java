
package com.longport.quote;

/**
 * Callback interface for real-time order book depth push events
 */
public interface DepthHandler {
    /**
     * Called when a depth update is received for the subscribed symbol.
     *
     * @param symbol security symbol
     * @param event  depth update event
     */
    void onDepth(String symbol, PushDepth event);
}
