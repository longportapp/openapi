#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_content_context_t lb_content_context_t;

namespace longport {
namespace content {

/// Content context
class ContentContext
{
private:
  const lb_content_context_t* ctx_;

public:
  ContentContext();
  ContentContext(const lb_content_context_t* ctx);
  ContentContext(const ContentContext& ctx);
  ContentContext(ContentContext&& ctx);
  ~ContentContext();

  ContentContext& operator=(const ContentContext& ctx);

  static ContentContext create(const Config& config);

  /// Get topics created by the current authenticated user
  void my_topics(const MyTopicsOptions& opts,
                   AsyncCallback<ContentContext, std::vector<OwnedTopic>> callback) const;

  /// Create a new topic
  void create_topic(const CreateTopicOptions& opts,
                    AsyncCallback<ContentContext, std::string> callback) const;

  /// Get discussion topics list for a symbol
  void topics(const std::string& symbol,
              AsyncCallback<ContentContext, std::vector<TopicItem>> callback) const;

  /// Get news list for a symbol
  void news(const std::string& symbol,
            AsyncCallback<ContentContext, std::vector<NewsItem>> callback) const;
};

} // namespace content
} // namespace longport
