package com.longport.content;

/**
 * Topic author
 */
public class TopicAuthor {
    private String memberId;
    private String name;
    private String avatar;

    /**
     * Returns the member ID.
     *
     * @return the member ID
     */
    public String getMemberId() {
        return memberId;
    }

    /**
     * Returns the display name.
     *
     * @return the display name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns the avatar URL.
     *
     * @return the avatar URL
     */
    public String getAvatar() {
        return avatar;
    }

    @Override
    public String toString() {
        return "TopicAuthor [memberId=" + memberId + ", name=" + name + ", avatar=" + avatar + "]";
    }
}
