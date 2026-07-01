package com.longport.market;

import com.longport.market.TopMoversEvent;

/** Response for {@link MarketContext#getTopMovers}. */
public class TopMoversResponse {
    /** Top mover events */
    public TopMoversEvent[] events;
    /** Pagination cursor (raw JSON); pass to next call for next page */
    public String nextParams;
}
