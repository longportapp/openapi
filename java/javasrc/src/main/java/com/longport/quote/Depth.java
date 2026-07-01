package com.longport.quote;

import java.math.BigDecimal;

/**
 * A single price level in the order book depth.
 */
public class Depth {
    private int position;
    private BigDecimal price;
    private long volume;
    private long orderNum;

    /**
     * Returns the position (1-based) in the order book.
     *
     * @return the position
     */
    public int getPosition() {
        return position;
    }

    /**
     * Returns the price at this level.
     *
     * @return the price
     */
    public BigDecimal getPrice() {
        return price;
    }

    /**
     * Returns the volume at this price level.
     *
     * @return the volume
     */
    public long getVolume() {
        return volume;
    }

    /**
     * Returns the number of orders at this price level.
     *
     * @return the number of orders
     */
    public long getOrderNum() {
        return orderNum;
    }

    @Override
    public String toString() {
        return "Depth [orderNum=" + orderNum + ", position=" + position + ", price=" + price + ", volume=" + volume
                + "]";
    }
}
