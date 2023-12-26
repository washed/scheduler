use super::Trigger;
use chrono::{DateTime, TimeZone};
use std::time::Duration;

#[derive(Clone)]
pub struct Interval<Tz: TimeZone> {
    interval: std::time::Duration,
    now: fn() -> DateTime<Tz>,
    last_run: Option<DateTime<Tz>>,
}

impl<Tz: TimeZone> Interval<Tz> {
    pub fn new(interval: std::time::Duration, now: fn() -> DateTime<Tz>) -> Self {
        Self { interval, now, last_run: None }
    }
}

impl<Tz: TimeZone> Trigger<Tz> for Interval<Tz> {
    fn next_runs(&self, n: usize) -> Option<Vec<DateTime<Tz>>> {
        let now = (self.now)();
        let interval_millis = self.interval.as_millis() as u64;

        let last_run = match &self.last_run {
            Some(last_run_inner) => {
                let time_passed = now - last_run_inner.clone();
                let intervals_passed: u64 = (time_passed.to_std().unwrap().as_millis() / self.interval.as_millis()).try_into().unwrap();
                last_run_inner.clone() + Duration::from_millis((intervals_passed) * interval_millis)
            },
            None => now,
        };

        Some((1..n).map(move |i|
            last_run.clone() + Duration::from_millis(i as u64 * interval_millis)
        ).collect())
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
