package com.longport.trade;

import java.util.Arrays;

/**
 * Stock positions grouped by account channel
 */
public class StockPositionChannel {
    private String accountChannel;
    private StockPosition[] positions;

    /**
     * Returns the account channel identifier.
     *
     * @return account channel identifier
     */
    public String getAccountChannel() {
        return accountChannel;
    }

    /**
     * Returns the stock positions for this channel.
     *
     * @return stock positions
     */
    public StockPosition[] getPositions() {
        return positions;
    }

    @Override
    public String toString() {
        return "StockPositionChannel [accountChannel=" + accountChannel + ", positions=" + Arrays.toString(positions)
                + "]";
    }
}
