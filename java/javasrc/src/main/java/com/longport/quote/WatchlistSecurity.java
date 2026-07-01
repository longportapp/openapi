package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

import com.longport.Market;

/**
 * A security in a watchlist group.
 */
public class WatchlistSecurity {
    private String symbol;
    private Market market;
    private String name;
    private BigDecimal watchedPrice;
    private OffsetDateTime watchedAt;

    /**
     * Returns the security code.
     *
     * @return the security code
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the market the security belongs to.
     *
     * @return the market
     */
    public Market getMarket() {
        return market;
    }

    /**
     * Returns the security name.
     *
     * @return the security name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns the price at which the security was added to the watchlist, or null if not set.
     *
     * @return the watched price, or null if not set
     */
    public BigDecimal getWatchedPrice() {
        return watchedPrice;
    }

    /**
     * Returns the time at which the security was added to the watchlist.
     *
     * @return the time added to the watchlist
     */
    public OffsetDateTime getWatchedAt() {
        return watchedAt;
    }

    @Override
    public String toString() {
        return "WatchlistSecurity [market=" + market + ", name=" + name + ", watchedPrice=" + watchedPrice + ", symbol="
                + symbol
                + ", watchedAt=" + watchedAt + "]";
    }

}
