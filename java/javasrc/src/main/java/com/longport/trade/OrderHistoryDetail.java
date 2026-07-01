package com.longport.trade;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * A single entry in the order history
 */
public class OrderHistoryDetail {
    private BigDecimal price;
    private BigDecimal quantity;
    private OrderStatus status;
    private String msg;
    private OffsetDateTime time;

    /**
     * Returns the price at this history point.
     *
     * @return price
     */
    public BigDecimal getPrice() {
        return price;
    }

    /**
     * Returns the quantity at this history point.
     *
     * @return quantity
     */
    public BigDecimal getQuantity() {
        return quantity;
    }

    /**
     * Returns the order status at this history point.
     *
     * @return order status
     */
    public OrderStatus getStatus() {
        return status;
    }

    /**
     * Returns the message associated with this history entry.
     *
     * @return message
     */
    public String getMsg() {
        return msg;
    }

    /**
     * Returns the time of this history entry.
     *
     * @return time
     */
    public OffsetDateTime getTime() {
        return time;
    }

    @Override
    public String toString() {
        return "OrderHistoryDetail [price=" + price + ", quantity=" + quantity + ", status=" + status + ", msg=" + msg
                + ", time=" + time + "]";
    }

}
