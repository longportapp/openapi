package com.longport.trade;

import java.util.Arrays;

/**
 * Fund positions grouped by account channel
 */
public class FundPositionChannel {
    private String accountChannel;
    private FundPosition[] positions;

    /**
     * Returns the account channel identifier.
     *
     * @return account channel identifier
     */
    public String getAccountChannel() {
        return accountChannel;
    }

    /**
     * Returns the fund positions for this channel.
     *
     * @return fund positions
     */
    public FundPosition[] getPositions() {
        return positions;
    }

    @Override
    public String toString() {
        return "FundPositionChannel [accountChannel=" + accountChannel + ", positions=" + Arrays.toString(positions)
                + "]";
    }
}
