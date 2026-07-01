package com.longport.quote;

/**
 * Request object for updating a watchlist group
 */
@SuppressWarnings("unused")
public class UpdateWatchlistGroup {
    private long id;
    private String name;
    private String[] securities;
    private SecuritiesUpdateMode mode;

    /**
     * Constructs an update-watchlist-group request.
     *
     * @param id group ID to update
     */
    public UpdateWatchlistGroup(long id) {
        this.id = id;
        this.mode = SecuritiesUpdateMode.Replace;
    }

    /**
     * Sets the new group name.
     *
     * @param name new name
     * @return this instance for chaining
     */
    public UpdateWatchlistGroup setName(String name) {
        this.name = name;
        return this;
    }

    /**
     * Sets the securities in the group.
     *
     * @param securities security symbols
     * @return this instance for chaining
     */
    public UpdateWatchlistGroup setSecurities(String[] securities) {
        this.securities = securities;
        return this;
    }

    /**
     * Sets the update mode (add, remove, or replace).
     *
     * @param mode update mode
     * @return this instance for chaining
     */
    public UpdateWatchlistGroup setMode(SecuritiesUpdateMode mode) {
        this.mode = mode;
        return this;
    }
}
