package com.longport.content;

import java.time.OffsetDateTime;

/**
 * Topic created by the current authenticated user
 */
public class OwnedTopic {
    private String id;
    private String title;
    private String description;
    private String body;
    private TopicAuthor author;
    private String[] tickers;
    private String[] hashtags;
    private TopicImage[] images;
    private int likesCount;
    private int commentsCount;
    private int viewsCount;
    private int sharesCount;
    private String topicType;
    private String detailUrl;
    private OffsetDateTime createdAt;
    private OffsetDateTime updatedAt;

    /** Returns the topic ID. */
    public String getId() { return id; }

    /** Returns the title. */
    public String getTitle() { return title; }

    /** Returns the plain text excerpt. */
    public String getDescription() { return description; }

    /** Returns the Markdown body. */
    public String getBody() { return body; }

    /** Returns the author. */
    public TopicAuthor getAuthor() { return author; }

    /** Returns the related stock tickers. */
    public String[] getTickers() { return tickers; }

    /** Returns the hashtag names. */
    public String[] getHashtags() { return hashtags; }

    /** Returns the images. */
    public TopicImage[] getImages() { return images; }

    /** Returns the likes count. */
    public int getLikesCount() { return likesCount; }

    /** Returns the comments count. */
    public int getCommentsCount() { return commentsCount; }

    /** Returns the views count. */
    public int getViewsCount() { return viewsCount; }

    /** Returns the shares count. */
    public int getSharesCount() { return sharesCount; }

    /** Returns the content type: "article" or "post". */
    public String getTopicType() { return topicType; }

    /** Returns the URL to the full topic page. */
    public String getDetailUrl() { return detailUrl; }

    /** Returns the created time. */
    public OffsetDateTime getCreatedAt() { return createdAt; }

    /** Returns the updated time. */
    public OffsetDateTime getUpdatedAt() { return updatedAt; }

    @Override
    public String toString() {
        return "OwnedTopic [id=" + id + ", title=" + title + ", topicType=" + topicType
                + ", createdAt=" + createdAt + "]";
    }
}
