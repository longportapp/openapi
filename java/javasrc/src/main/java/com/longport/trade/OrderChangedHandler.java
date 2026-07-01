package com.longport.trade;

/**
 * Callback interface for order change push events
 */
public interface OrderChangedHandler {
    /**
     * Called when an order status change is received.
     *
     * @param orderChanged order change event
     */
    void onOrderChanged(PushOrderChanged orderChanged);
}
