use time::{Date, Month, OffsetDateTime, Time};
use time_tz::PrimitiveDateTimeExt;

use crate::{Market, Period, TradeSessionKind};

pub struct TestCandlestickTime<'a> {
    market: &'a Market,
    period: Period,
}

impl<'a> TestCandlestickTime<'a> {
    #[inline]
    pub fn new(market: &'a Market, period: Period) -> Self {
        Self { market, period }
    }

    #[track_caller]
    pub fn check_time(&self, ts: TradeSessionKind, input: Time, expected: impl Into<Option<Time>>) {
        let date = Date::from_calendar_date(2024, Month::January, 1).unwrap();
        assert_eq!(
            self.market.candlestick_time(
                ts,
                false,
                self.period,
                date.with_time(input)
                    .assume_timezone(self.market.timezone)
                    .unwrap_first(),
            ),
            expected.into().map(|expected| date
                .with_time(expected)
                .assume_timezone(self.market.timezone)
                .unwrap_first())
        );
    }

    #[track_caller]
    pub fn check_datetime(
        &self,
        ts: TradeSessionKind,
        input: OffsetDateTime,
        expected: impl Into<Option<OffsetDateTime>>,
    ) {
        assert_eq!(
            self.market.candlestick_time(ts, false, self.period, input),
            expected.into()
        );
    }
}
