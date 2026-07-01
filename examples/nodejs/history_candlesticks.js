const {
  Config,
  QuoteContext,
  Period,
  AdjustType,
  TradeSessions,
  NaiveDate,
  NaiveDatetime,
  Time,
  OAuth,
} = require('longport');

async function main() {
  const oauth = await OAuth.build("your-client-id", (_, url) => {
    console.log("Open this URL to authorize: " + url);
  });
  const config = Config.fromOAuth(oauth);
  const ctx = QuoteContext.new(config);

  // get candlesticks by offset
  console.log("get candlesticks by offset");
  console.log("====================");
  const datetime = new NaiveDatetime(new NaiveDate(2023, 8, 18), new Time(0, 0, 0));
  const byOffset = await ctx.historyCandlesticksByOffset(
    "700.HK",
    Period.Day,
    AdjustType.NoAdjust,
    false,
    datetime,
    10,
    TradeSessions.Intraday,
  );
  for (const candlestick of byOffset) {
    console.log(candlestick.toString());
  }

  // get candlesticks by date
  console.log("get candlesticks by date");
  console.log("====================");
  const byDate = await ctx.historyCandlesticksByDate(
    "700.HK",
    Period.Day,
    AdjustType.NoAdjust,
    new NaiveDate(2022, 5, 5),
    new NaiveDate(2022, 6, 23),
    TradeSessions.Intraday,
  );
  for (const candlestick of byDate) {
    console.log(candlestick.toString());
  }
}

main().catch(console.error);
