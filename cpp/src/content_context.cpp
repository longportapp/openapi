#include "content_context.hpp"
#include "convert.hpp"
#include <algorithm>
#include <iterator>

namespace longport {
namespace content {

using longport::convert::convert;

ContentContext::ContentContext()
  : ctx_(nullptr)
{
}

ContentContext::ContentContext(const lb_content_context_t* ctx)
{
  ctx_ = ctx;
  if (ctx_) {
    lb_content_context_retain(ctx_);
  }
}

ContentContext::ContentContext(const ContentContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_content_context_retain(ctx_);
  }
}

ContentContext::ContentContext(ContentContext&& ctx)
{
  ctx_ = ctx.ctx_;
  ctx.ctx_ = nullptr;
}

ContentContext::~ContentContext()
{
  if (ctx_) {
    lb_content_context_release(ctx_);
  }
}

ContentContext&
ContentContext::operator=(const ContentContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_content_context_retain(ctx_);
  }
  return *this;
}

ContentContext
ContentContext::create(const Config& config)
{
  auto* ctx_ptr = lb_content_context_new(config);
  ContentContext ctx(ctx_ptr);
  if (ctx_ptr) {
    lb_content_context_release(ctx_ptr);
  }
  return ctx;
}

void
ContentContext::my_topics(
  const MyTopicsOptions& opts,
  AsyncCallback<ContentContext, std::vector<OwnedTopic>> callback) const
{
  const char* topic_type =
    opts.topic_type.empty() ? nullptr : opts.topic_type.c_str();
  lb_content_context_my_topics(
    ctx_,
    opts.page,
    opts.size,
    topic_type,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<ContentContext, std::vector<OwnedTopic>>(
          res->userdata);
      ContentContext ctx((const lb_content_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_owned_topic_t*)res->data;
        std::vector<OwnedTopic> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(
          AsyncResult<ContentContext, std::vector<OwnedTopic>>(
            ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(
          AsyncResult<ContentContext, std::vector<OwnedTopic>>(
            ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<ContentContext, std::vector<OwnedTopic>>(callback));
}

void
ContentContext::create_topic(
  const CreateTopicOptions& opts,
  AsyncCallback<ContentContext, std::string> callback) const
{
  const char* topic_type =
    opts.topic_type.empty() ? nullptr : opts.topic_type.c_str();
  std::vector<const char*> tickers_cstr;
  for (const auto& t : opts.tickers) {
    tickers_cstr.push_back(t.c_str());
  }
  std::vector<const char*> hashtags_cstr;
  for (const auto& h : opts.hashtags) {
    hashtags_cstr.push_back(h.c_str());
  }
  lb_content_context_create_topic(
    ctx_,
    opts.title.c_str(),
    opts.body.c_str(),
    topic_type,
    tickers_cstr.empty() ? nullptr : tickers_cstr.data(),
    tickers_cstr.size(),
    hashtags_cstr.empty() ? nullptr : hashtags_cstr.data(),
    hashtags_cstr.size(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<ContentContext, std::string>(
          res->userdata);
      ContentContext ctx((const lb_content_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        std::string result((const char*)res->data);

        (*callback_ptr)(
          AsyncResult<ContentContext, std::string>(
            ctx, std::move(status), &result));
      } else {
        (*callback_ptr)(
          AsyncResult<ContentContext, std::string>(
            ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<ContentContext, std::string>(callback));
}

void
ContentContext::topics(
  const std::string& symbol,
  AsyncCallback<ContentContext, std::vector<TopicItem>> callback) const
{
  lb_content_context_topics(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<ContentContext, std::vector<TopicItem>>(
          res->userdata);
      ContentContext ctx((const lb_content_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_topic_item_t*)res->data;
        std::vector<TopicItem> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<ContentContext, std::vector<TopicItem>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<ContentContext, std::vector<TopicItem>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<ContentContext, std::vector<TopicItem>>(callback));
}

void
ContentContext::news(
  const std::string& symbol,
  AsyncCallback<ContentContext, std::vector<NewsItem>> callback) const
{
  lb_content_context_news(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<ContentContext, std::vector<NewsItem>>(
          res->userdata);
      ContentContext ctx((const lb_content_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_news_item_t*)res->data;
        std::vector<NewsItem> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<ContentContext, std::vector<NewsItem>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<ContentContext, std::vector<NewsItem>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<ContentContext, std::vector<NewsItem>>(callback));
}

} // namespace content
} // namespace longport
