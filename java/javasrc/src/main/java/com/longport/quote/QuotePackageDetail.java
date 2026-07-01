package com.longport.quote;

import java.time.OffsetDateTime;

/**
 * Quote package subscription detail.
 */
public class QuotePackageDetail {
    private String key;
    private String name;
    private String description;
    private OffsetDateTime startAt;
    private OffsetDateTime endAt;

    /**
     * Returns the package key identifier.
     *
     * @return the package key identifier
     */
    public String getKey() {
        return key;
    }

    /**
     * Returns the package name.
     *
     * @return the package name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns the package description.
     *
     * @return the package description
     */
    public String getDescription() {
        return description;
    }

    /**
     * Returns the start time of the package subscription.
     *
     * @return the start time of the package subscription
     */
    public OffsetDateTime getStartAt() {
        return startAt;
    }

    /**
     * Returns the end time of the package subscription.
     *
     * @return the end time of the package subscription
     */
    public OffsetDateTime getEndAt() {
        return endAt;
    }

    @Override
    public String toString() {
        return "QuotePackageDetail [key=" + key + ", name=" + name + ", description=" + description + ", startAt="
                + startAt + ", endAt=" + endAt + "]";
    }
}