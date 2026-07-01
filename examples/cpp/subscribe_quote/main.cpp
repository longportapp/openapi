#include <iostream>
#include <longport.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longport;
using namespace longport::quote;

static QuoteContext g_ctx;

static void
run(const OAuth& oauth)
{
  Config config = Config::from_oauth(oauth);
  g_ctx = QuoteContext::create(config);

  g_ctx.set_on_quote([](auto event) {
    std::cout << event->symbol << " timestamp=" << event->timestamp
              << " last_done=" << (double)event->last_done
              << " open=" << (double)event->open
              << " high=" << (double)event->high
              << " low=" << (double)event->low
              << " volume=" << event->volume
              << " turnover=" << (double)event->turnover << std::endl;
  });

  std::vector<std::string> symbols = {
    "700.HK", "AAPL.US", "TSLA.US", "NFLX.US"
  };

  g_ctx.subscribe(symbols, SubFlags::QUOTE(), [](auto res) {
    if (!res) {
      std::cout << "failed to subscribe quote: "
                << *res.status().message() << std::endl;
      return;
    }
  });
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  const std::string client_id = "your-client-id";

  OAuthBuilder(client_id).build(
    [](const std::string& url) {
      std::cout << "Open this URL to authorize: " << url << std::endl;
    },
    [](auto res) {
      if (!res) {
        std::cout << "authorization failed: " << *res.status().message()
                  << std::endl;
        return;
      }
      run(*res);
    });

  std::cin.get();
  return 0;
}
