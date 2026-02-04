"""Submit order (async). Use asyncio with AsyncTradeContext."""
import asyncio
from decimal import Decimal

from longport.openapi import (
    AsyncTradeContext,
    Config,
    OrderSide,
    OrderType,
    OutsideRTH,
    TimeInForceType,
)


async def main() -> None:
    config = Config.from_env()
    ctx = await AsyncTradeContext.create(config)
    resp = await ctx.submit_order(
        symbol="700.HK",
        order_type=OrderType.MO,
        side=OrderSide.Buy,
        submitted_quantity=Decimal(200),
        time_in_force=TimeInForceType.Day,
        outside_rth=OutsideRTH.AnyTime,
        remark="Hello from Python SDK",
    )
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
