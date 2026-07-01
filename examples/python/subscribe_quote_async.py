"""Subscribe quote (async). Use asyncio with AsyncQuoteContext."""
import asyncio

from longport.openapi import AsyncQuoteContext, Config, OAuthBuilder, SubType, PushQuote


async def on_quote(symbol: str, event: PushQuote) -> None:
    """Callback may be sync or async; async callbacks are scheduled on the event loop."""
    print(symbol, event)


async def main() -> None:
    oauth = await OAuthBuilder("your-client-id").build_async(
        lambda url: print(f"Open this URL to authorize: {url}")
    )
    config = Config.from_oauth(oauth)
    # Pass the event loop so async callbacks (e.g. async def on_quote) are scheduled.
    ctx = AsyncQuoteContext.create(config, loop_=asyncio.get_running_loop())
    ctx.set_on_quote(on_quote)
    await ctx.subscribe(
        ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"],
        [SubType.Quote],
    )
    await asyncio.sleep(10)


if __name__ == "__main__":
    asyncio.run(main())
