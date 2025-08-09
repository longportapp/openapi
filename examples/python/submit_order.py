from decimal import Decimal
from longport.openapi import TradeContext, Config, OrderSide, OrderType, TimeInForceType, OutsideRTH

config = Config.from_env()
ctx = TradeContext(config)

resp = ctx.submit_order(
    side=OrderSide.Buy,
    symbol="700.HK",
    order_type=OrderType.MO,
    submitted_quantity=Decimal(200),
    time_in_force=TimeInForceType.Day,
    outside_rth=OutsideRTH.AnyTime,
    remark="Hello from Python SDK",
)
print(resp)
