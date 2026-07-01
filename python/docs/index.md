# Longport OpenAPI SDK for Python

`longport` provides an easy-to-use interface for invokes [`Longport OpenAPI`](https://open.longportapp.com/en/).

## References

- [Config](reference_all.md#longport.openapi.Config)

  The configuration of the SDK.
   
- [QuoteContext](reference_all.md#longport.openapi.QuoteContext)

  The Quote API part of the SDK, e.g.: get basic information of securities, subscribe quotes...

- [TradeContext](reference_all.md#longport.openapi.TradeContext)

  The Trade API part of the SDK, e.g.: submit order, get order status...

- [AsyncQuoteContext](reference_all.md#longport.openapi.AsyncQuoteContext)

  Async quote API for use with asyncio; create via `AsyncQuoteContext.create(config)` and await in asyncio.

- [AsyncTradeContext](reference_all.md#longport.openapi.AsyncTradeContext)

  Async trade API for use with asyncio; create via `AsyncTradeContext.create(config)` and await in asyncio.

- [ContentContext](reference_all.md#longport.openapi.ContentContext)

  The Content API part of the SDK, e.g.: get news, discussion topics for a security.

- [AsyncContentContext](reference_all.md#longport.openapi.AsyncContentContext)

  Async content API for use with asyncio; create via `AsyncContentContext.create(config)` (synchronous, no await needed at construction).

## Quickstart

_Install Longport OpenAPI SDK_

```bash
pip install longport
```

### Authentication

#### OAuth 2.0 (Recommended)

```python
from longport.openapi import OAuthBuilder, Config

oauth = OAuthBuilder("your-client-id").build(
    lambda url: print(f"Open this URL to authorize: {url}")
)
config = Config.from_oauth(oauth)
```

#### Legacy API Key (Environment Variables)

_macOS/Linux_

```bash
export LONGPORT_APP_KEY="App Key get from user center"
export LONGPORT_APP_SECRET="App Secret get from user center"
export LONGPORT_ACCESS_TOKEN="Access Token get from user center"
```

_Windows_

```powershell
setx LONGPORT_APP_KEY "App Key get from user center"
setx LONGPORT_APP_SECRET "App Secret get from user center"
setx LONGPORT_ACCESS_TOKEN "Access Token get from user center"
```

```python
from longport.openapi import Config

config = Config.from_apikey_env()
```

## Quote API _(Get basic information of securities)_

```python
from longport.openapi import Config, QuoteContext, OAuthBuilder

oauth = OAuthBuilder("your-client-id").build(
    lambda url: print(f"Open this URL to authorize: {url}")
)
config = Config.from_oauth(oauth)

# Create a context for quote APIs
ctx = QuoteContext(config)

# Get basic information of securities
resp = ctx.quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
print(resp)
```

## Quote API _(Subscribe quotes)_

```python
from time import sleep
from longport.openapi import Config, QuoteContext, SubType, PushQuote, OAuthBuilder

oauth = OAuthBuilder("your-client-id").build(
    lambda url: print(f"Open this URL to authorize: {url}")
)
config = Config.from_oauth(oauth)

# A callback to receive quote data
def on_quote(symbol: str, event: PushQuote):
    print(symbol, event)

# Create a context for quote APIs
ctx = QuoteContext(config)
ctx.set_on_quote(on_quote)

# Subscribe
ctx.subscribe(["700.HK"], [SubType.Quote])

# Receive push for 30 seconds
sleep(30)
```

## Trade API _(Submit order)_

```python
from decimal import Decimal
from longport.openapi import TradeContext, Config, OrderType, OrderSide, TimeInForceType, OAuthBuilder

oauth = OAuthBuilder("your-client-id").build(
    lambda url: print(f"Open this URL to authorize: {url}")
)
config = Config.from_oauth(oauth)

# Create a context for trade APIs
ctx = TradeContext(config)

# Submit order
resp = ctx.submit_order(
    "700.HK", OrderType.LO, OrderSide.Buy,
    Decimal("500"), TimeInForceType.Day,
    submitted_price=Decimal("50"),
    remark="Hello from Python SDK",
)
print(resp)
```

## Content API _(Get news and topics for a security)_

```python
from longport.openapi import Config, ContentContext

config = Config.from_apikey_env()

# Create a context for content APIs
ctx = ContentContext(config)

# Get news for a security
news = ctx.news("700.HK")
print(news)

# Get discussion topics for a security
topics = ctx.topics("700.HK")
print(topics)
```

## Asynchronous API

The SDK provides async contexts and an async HTTP client for use with Python's `asyncio`. All I/O methods return awaitables; callbacks (e.g. for push events) are set the same way as in the sync API.

```python
import asyncio
from longport.openapi import Config, AsyncQuoteContext, SubType, PushQuote, OAuthBuilder

def on_quote(symbol: str, event: PushQuote):
    print(symbol, event)

async def main():
    oauth = await OAuthBuilder("your-client-id").build_async(
        lambda url: print(f"Open this URL to authorize: {url}")
    )
    config = Config.from_oauth(oauth)
    ctx = AsyncQuoteContext.create(config)
    ctx.set_on_quote(on_quote)
    await ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote])
    quotes = await ctx.quote(["700.HK"])
    print(quotes)
    await asyncio.sleep(10)

asyncio.run(main())
```

See the `*_async.py` examples in the repo and the reference for `AsyncQuoteContext`, `AsyncTradeContext`, and `HttpClient.request_async`.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license [LICENSE-MIT](http://opensource.org/licenses/MIT) at your option.
