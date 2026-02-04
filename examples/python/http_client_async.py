"""HTTP client (async). Use asyncio with HttpClient.request_async."""
import asyncio

from longport.openapi import HttpClient


async def main() -> None:
    http_cli = HttpClient.from_env()
    resp = await http_cli.request_async(
        "get",
        "/v1/trade/execution/today",
    )
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
