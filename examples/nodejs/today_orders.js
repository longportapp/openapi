const { Config, TradeContext, OAuth } = require('longport');

async function main() {
  const oauth = await OAuth.build("your-client-id", (_, url) => {
    console.log("Open this URL to authorize: " + url);
  });
  let config = Config.fromOAuth(oauth);
  let ctx = TradeContext.new(config);
  let resp = await ctx.todayOrders();
  for (let obj of resp) {
    console.log(obj.toString());
  }
}

Promise.all([main()]).catch((err) => console.error(err));
