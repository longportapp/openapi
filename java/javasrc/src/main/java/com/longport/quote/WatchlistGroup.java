package com.longport.quote;

import java.util.Arrays;

/**
 * Watchlist group.
 */
public class WatchlistGroup {
    private long id;
    private String name;
    private WatchlistSecurity[] securities;

    /**
     * Returns the group ID.
     *
     * @return the group ID
     */
    public long getId() {
        return id;
    }

    /**
     * Returns the group name.
     *
     * @return the group name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns the securities in this group.
     *
     * @return the securities in this group
     */
    public WatchlistSecurity[] getSecurities() {
        return securities;
    }

    @Override
    public String toString() {
        return "WatchlistGroup [id=" + id + ", name=" + name + ", securities=" + Arrays.toString(securities) + "]";
    }
}
