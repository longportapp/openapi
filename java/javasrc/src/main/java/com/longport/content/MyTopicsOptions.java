package com.longport.content;

/**
 * Options for listing topics created by the current authenticated user
 */
@SuppressWarnings("unused")
public class MyTopicsOptions {
    private Integer page;
    private Integer size;
    private String topicType;

    /**
     * Sets the page number (default 1).
     *
     * @param page page number
     * @return this instance for chaining
     */
    public MyTopicsOptions setPage(int page) {
        this.page = page;
        return this;
    }

    /**
     * Sets the number of records per page, range 1~500 (default 50).
     *
     * @param size records per page
     * @return this instance for chaining
     */
    public MyTopicsOptions setSize(int size) {
        this.size = size;
        return this;
    }

    /**
     * Filters by topic type: "article" or "post". Leave null to return all.
     *
     * @param topicType topic type filter
     * @return this instance for chaining
     */
    public MyTopicsOptions setTopicType(String topicType) {
        this.topicType = topicType;
        return this;
    }
}
