package com.longport.content;

/**
 * Topic image
 */
public class TopicImage {
    private String url;
    private String sm;
    private String lg;

    /**
     * Returns the original image URL.
     *
     * @return the original image URL
     */
    public String getUrl() {
        return url;
    }

    /**
     * Returns the small thumbnail URL.
     *
     * @return the small thumbnail URL
     */
    public String getSm() {
        return sm;
    }

    /**
     * Returns the large image URL.
     *
     * @return the large image URL
     */
    public String getLg() {
        return lg;
    }

    @Override
    public String toString() {
        return "TopicImage [url=" + url + ", sm=" + sm + ", lg=" + lg + "]";
    }
}
