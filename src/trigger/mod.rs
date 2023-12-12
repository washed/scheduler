pub mod oneshot;
mod tests;
pub mod weekly;

use chrono::{DateTime, Duration, TimeZone};

pub trait Trigger<Tz: TimeZone> {
    fn next_runs(&self, _n: usize) -> Vec<DateTime<Tz>> {
        Vec::<DateTime<Tz>>::new()
    }

    fn time_to_next_runs(&self, _n: usize) -> Vec<Duration> {
        Vec::<Duration>::new()
    }
}
