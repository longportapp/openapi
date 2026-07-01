package com.longport.quote;

import java.time.LocalTime;

/**
 * Time range of a single trading session.
 */
public class TradingSessionInfo {
    private LocalTime beginTime;
    private LocalTime endTime;
    private TradeSession tradeSession;

    /**
     * Returns the start time of this session.
     *
     * @return the start time of this session
     */
    public LocalTime getBeginTime() {
        return beginTime;
    }

    /**
     * Returns the end time of this session.
     *
     * @return the end time of this session
     */
    public LocalTime getEndTime() {
        return endTime;
    }

    /**
     * Returns the trade session type.
     *
     * @return the trade session type
     */
    public TradeSession getTradeSession() {
        return tradeSession;
    }

    @Override
    public String toString() {
        return "TradingSessionInfo [beginTime=" + beginTime + ", endTime=" + endTime + ", tradeSession=" + tradeSession
                + "]";
    }
}
