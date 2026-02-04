"""Get today orders (async). Use asyncio with AsyncTradeContext."""
import asyncio

from longport.openapi import AsyncTradeContext, Config


async def main() -> None:
    config = Config.from_env()
    ctx = await AsyncTradeContext.create(config)
    resp = await ctx.today_orders()
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
