package com.longport.quote;

/**
 * Request object for deleting a watchlist group
 */
@SuppressWarnings("unused")
public class DeleteWatchlistGroup {
    private long id;
    private boolean purge;

    /**
     * Constructs a delete-watchlist-group request.
     *
     * @param id group ID to delete
     */
    public DeleteWatchlistGroup(long id) {
        this.id = id;
        this.purge = false;
    }

    /**
     * Sets the purge flag, which also removes all securities from the group before deletion.
     *
     * @return this instance for chaining
     */
    public DeleteWatchlistGroup purge() {
        this.purge = true;
        return this;
    }
}
