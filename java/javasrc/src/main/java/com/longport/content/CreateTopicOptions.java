package com.longport.content;

/**
 * Options for creating a topic
 */
@SuppressWarnings("unused")
public class CreateTopicOptions {
    private String title;
    private String body;
    private String topicType;
    private String[] tickers;
    private String[] hashtags;

    /**
     * Constructs a create-topic request.
     *
     * @param title topic title (required)
     * @param body  topic body in Markdown format (required)
     */
    public CreateTopicOptions(String title, String body) {
        this.title = title;
        this.body = body;
    }

    /**
     * Sets the content type: "article" (long-form) or "post" (short post, default).
     *
     * @param topicType content type
     * @return this instance for chaining
     */
    public CreateTopicOptions setTopicType(String topicType) {
        this.topicType = topicType;
        return this;
    }

    /**
     * Sets the related stock tickers, format: {symbol}.{market}, max 10.
     *
     * @param tickers stock tickers
     * @return this instance for chaining
     */
    public CreateTopicOptions setTickers(String[] tickers) {
        this.tickers = tickers;
        return this;
    }

    /**
     * Sets the hashtag names, max 5.
     *
     * @param hashtags hashtag names
     * @return this instance for chaining
     */
    public CreateTopicOptions setHashtags(String[] hashtags) {
        this.hashtags = hashtags;
        return this;
    }

}
