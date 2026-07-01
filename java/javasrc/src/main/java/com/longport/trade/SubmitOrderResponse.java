package com.longport.trade;

/**
 * Response from submitting an order
 */
public class SubmitOrderResponse {
    private String orderId;

    /**
     * Returns the order ID of the submitted order.
     *
     * @return order ID
     */
    public String getOrderId() {
        return orderId;
    }

    @Override
    public String toString() {
        return "SubmitOrderResponse [orderId=" + orderId + "]";
    }
}
