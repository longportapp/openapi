# Longport OpenAPI SDK

[![](https://img.shields.io/crates/v/longport.svg)](https://crates.io/crates/longport) [![PyPI version](https://badge.fury.io/py/longport.svg)](https://badge.fury.io/py/longport) [![npm version](https://badge.fury.io/js/longport.svg)](https://badge.fury.io/js/longport) [![Maven Central](https://img.shields.io/maven-central/v/io.github.longportapp/openapi-sdk)](https://search.maven.org/artifact/io.github.longportapp/openapi-sdk)


Longport OpenAPI provides programmatic quote trading interfaces for investors with research and development capabilities and assists them to build trading or quote strategy analysis tools based on their own investment strategies. The functions fall into the following categories:

- Trading - Create, amend, cancel orders, query today’s/past orders and transaction details, etc.
- Quotes - Real-time quotes, acquisition of historical quotes, etc.
- Portfolio - Real-time query of the account assets, positions, funds
- Real-time subscription - Provides real-time quotes and push notifications for order status changes

**This repo contains the following main components:**

| Name                        | Document                                                              | Description                                       |
|-----------------------------|-----------------------------------------------------------------------|---------------------------------------------------|
| [Rust](rust/README.md)      | [Doc](https://longportapp.github.io/openapi/rust/longport/index.html) | Longport OpenAPI for Rust `(>= 1.89.0)`           |
| [Python](python/README.md)  | [Doc](https://longportapp.github.io/openapi/python/index.html)        | Longport OpenAPI for Python 3 `(>= 3.8)`          |
| [Node.js](nodejs/README.md) | [Doc](https://longportapp.github.io/openapi/nodejs/index.html)        | Longport OpenAPI for Node.js `(>= 10)`            |
| [Java](java/README.md)      | [Doc](https://longportapp.github.io/openapi/java/index.html)          | Longport OpenAPI for Java `(>= 11)`               |
| [C](c/README.md)            | [Doc](https://longportapp.github.io/openapi/c/index.html)             | Longport OpenAPI for C `(>= C99)`                 |
| [C++](cpp/README.md)        | [Doc](https://longportapp.github.io/openapi/cpp/index.html)           | Longport OpenAPI for C++`(>= C++17)`              |
| Go                          |                                                                       | https://github.com/longportapp/openapi-go         |
| [MCP](mcp/README.md)        |                                                                       | An MCP server implementation for Longport OpenAPI |


## Context Types

| Context | Description |
|---------|-------------|
| `QuoteContext` | Real-time quotes, candlesticks, options, warrants, watchlists, push subscriptions |
| `TradeContext` | Orders, positions, account balance, executions, cash flow |
| `AssetContext` | Account statement download |
| `ContentContext` | News, community topics |
| `FundamentalContext` | Financial reports, analyst ratings, dividends, valuation, company overview, shareholders |
| `MarketContext` | Market status, broker holdings, A/H premium, trade statistics, anomaly alerts, index constituents |
| `CalendarContext` | Financial calendar (earnings, dividends, splits, IPOs, macro data, market closures) |
| `PortfolioContext` | Exchange rates, portfolio P&L analysis |
| `AlertContext` | Price alert management (add/enable/disable/delete) |
| `DCAContext` | Dollar-cost averaging plan management |
| `SharelistContext` | Community sharelist management |

## Quickstart

Pick a language SDK from the table above and follow its README for install and first request. Full reference docs: https://longportapp.github.io/openapi

## SDK Documentation

https://longportapp.github.io/openapi

## Troubleshooting

- Environment variables not taking effect
  - macOS/Linux: `export ...` only affects the current shell session.
  - Windows: `setx ...` requires opening a new terminal/session to take effect.
- Authentication errors (401/403)
  - Verify `LONGPORT_APP_KEY`, `LONGPORT_APP_SECRET`, `LONGPORT_ACCESS_TOKEN` are correct and not expired.
  - Ensure your OpenAPI app has the required permissions.
- Network / connection errors
  - Check firewall/proxy rules for HTTPS/WSS.
  - If you use a custom endpoint, set `LONGPORT_HTTP_URL`, `LONGPORT_QUOTE_WS_URL`, `LONGPORT_TRADE_WS_URL`.
- Quote subscription exits immediately
  - Keep the process running (event loop / sleep / blocking receive loop), otherwise you will not see push events.
- Debugging
  - Enable logs via `LONGPORT_LOG_PATH`.
  - If quotes connect but look empty, keep `LONGPORT_PRINT_QUOTE_PACKAGES=true` to confirm opened quote packages.

## Minimal Verification

If you're not sure whether your environment / credentials are correct, start with the built-in HTTP client examples.

- Python:

  ```bash
  python examples/python/http_client.py
  ```

- Node.js:

  ```bash
  node examples/nodejs/http_client.js
  ```

- Rust:

  ```bash
  cargo run --manifest-path examples/rust/Cargo.toml -p http_client
  ```

- Java (from the example module directory):

  ```bash
  cd examples/java/http_client
  mvn -q -DskipTests package
  mvn -q -DskipTests exec:java
  ```

- C/C++:
  - Use the sources in `examples/c/http_client/main.c` and `examples/cpp/http_client/main.cpp`.
  - Build instructions depend on your toolchain; see the corresponding language SDK README.

Expected results:

- If credentials are valid and network is reachable, the HTTP call returns JSON.
- If it returns 401/403, check your `LONGPORT_APP_KEY`, `LONGPORT_APP_SECRET`, `LONGPORT_ACCESS_TOKEN`.
- If it times out / cannot connect, check proxy/firewall and your endpoint env vars.

## Resources

- [Longport OpenAPI](https://open.longportapp.com/en/)
- [Longport OpenAPI Docs](https://open.longportapp.com/en/docs)

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
