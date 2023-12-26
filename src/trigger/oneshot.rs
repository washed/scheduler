use super::Trigger;
use chrono::{DateTime, TimeZone};
use std::time::Duration;

#[derive(Clone)]
pub struct Oneshot<Tz: TimeZone> {
    datetime: DateTime<Tz>,
    now: fn() -> DateTime<Tz>,
}

impl<Tz: TimeZone> Oneshot<Tz> {
    pub fn new(datetime: DateTime<Tz>, now: fn() -> DateTime<Tz>) -> Self {
        Self { datetime, now }
    }
}

impl<Tz: TimeZone> Trigger<Tz> for Oneshot<Tz> {
    fn next_runs(&self, _n: usize) -> Option<Vec<DateTime<Tz>>> {
        match self.datetime < (self.now)() {
            true => None,
            false => Some(vec![self.datetime.clone()]),
        }
    }

    fn time_to_next_runs(&self, n: usize) -> Option<Vec<Duration>> {
        let next_runs = self.next_runs(n)?;
        Some(
            next_runs
                .into_iter()
                .map(move |dt| {
                    let now = (self.now)();
                    (dt - now).to_std().unwrap()
                })
                .collect(),
        )
    }
}
