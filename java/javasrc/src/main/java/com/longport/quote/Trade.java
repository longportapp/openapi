package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * A single trade tick.
 */
public class Trade {
    private BigDecimal price;
    private long volume;
    private OffsetDateTime timestamp;
    private String tradeType;
    private TradeDirection direction;
    private TradeSession tradeSession;

    /**
     * Returns the trade price.
     *
     * @return the trade price
     */
    public BigDecimal getPrice() {
        return price;
    }

    /**
     * Returns the trade volume.
     *
     * @return the trade volume
     */
    public long getVolume() {
        return volume;
    }

    /**
     * Returns the time of the trade.
     *
     * @return the time of the trade
     */
    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    /**
     * Returns the exchange-specific trade type code.
     *
     * @return the trade type code
     */
    public String getTradeType() {
        return tradeType;
    }

    /**
     * Returns the trade direction (uptick / downtick / neutral).
     *
     * @return the trade direction
     */
    public TradeDirection getDirection() {
        return direction;
    }

    /**
     * Returns the trade session this trade occurred in.
     *
     * @return the trade session
     */
    public TradeSession getTradeSession() {
        return tradeSession;
    }

    @Override
    public String toString() {
        return "Trade [direction=" + direction + ", price=" + price + ", timestamp=" + timestamp + ", tradeType="
                + tradeType + ", tradeSession=" + tradeSession + ", volume=" + volume + "]";
    }
}
