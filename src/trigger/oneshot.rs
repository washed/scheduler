use super::Trigger;
use chrono::{DateTime, Duration, TimeZone};

pub struct Oneshot<Tz: TimeZone> {
    datetime: DateTime<Tz>,
    now: fn() -> DateTime<Tz>,

    pub callback: fn(dt: DateTime<Tz>),
}

impl<Tz: TimeZone> Oneshot<Tz> {
    pub fn new(
        datetime: DateTime<Tz>,
        callback: fn(dt: DateTime<Tz>),
        now: fn() -> DateTime<Tz>,
    ) -> Self {
        Self {
            datetime,
            callback,
            now,
        }
    }
}

impl<Tz: TimeZone> Trigger<Tz> for Oneshot<Tz> {
    fn next_runs(&self, _n: usize) -> Vec<DateTime<Tz>> {
        match self.datetime < (self.now)() {
            true => Vec::<DateTime<Tz>>::new(),
            false => vec![self.datetime.clone()],
        }
    }

    fn time_to_next_runs(&self, n: usize) -> Vec<Duration> {
        let next_runs = self.next_runs(n);
        next_runs
            .into_iter()
            .map(move |dt| {
                let now = (self.now)();
                dt - now
            })
            .collect()
    }
}
