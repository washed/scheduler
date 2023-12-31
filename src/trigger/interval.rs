use super::{NowUtc, Trigger};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
// use std::fmt::Debug;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Interval {
    interval: std::time::Duration,
    last_run: Option<DateTime<Utc>>,
}

impl Interval {
    pub fn new(interval: std::time::Duration) -> Self {
        Self {
            interval,
            last_run: None,
        }
    }
}

#[cfg(not(test))]
impl NowUtc for Interval {}

#[typetag::serde]
impl Trigger for Interval {
    fn next_runs(&self, n: usize) -> Option<Vec<DateTime<Utc>>> {
        let now = Self::now_utc();
        let interval_millis = self.interval.as_millis() as u64;

        let last_run = match &self.last_run {
            Some(last_run_inner) => {
                let time_passed = now - last_run_inner;
                let intervals_passed: u64 = (time_passed.to_std().unwrap().as_millis()
                    / self.interval.as_millis())
                .try_into()
                .unwrap();
                *last_run_inner + Duration::from_millis((intervals_passed) * interval_millis)
            }
            None => now,
        };

        Some(
            (1..n + 1)
                .map(move |i| last_run + Duration::from_millis(i as u64 * interval_millis))
                .collect(),
        )
    }

    fn time_to_next_runs(&self, n: usize) -> Option<Vec<Duration>> {
        let next_runs = self.next_runs(n)?;
        Some(
            next_runs
                .into_iter()
                .map(move |dt| {
                    let now = Utc::now();
                    (dt - now).to_std().unwrap()
                })
                .collect(),
        )
    }

    fn hash(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
