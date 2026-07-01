package com.longport.quote;

/**
 * Request object for creating a new watchlist group
 */
@SuppressWarnings("unused")
public class CreateWatchlistGroup {
    private String name;
    private String[] securities;

    /**
     * Constructs a create-watchlist-group request.
     *
     * @param name group name
     */
    public CreateWatchlistGroup(String name) {
        this.name = name;
    }

    /**
     * Sets the initial securities to add to the group.
     *
     * @param securities security symbols
     * @return this instance for chaining
     */
    public CreateWatchlistGroup setSecurities(String[] securities) {
        this.securities = securities;
        return this;
    }
}
