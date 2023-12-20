pub mod oneshot;
mod tests;
pub mod weekly;

use chrono::{DateTime, Duration, TimeZone};

pub trait Trigger<Tz: TimeZone> {
    fn next_runs(&self, _n: usize) -> Option<Vec<DateTime<Tz>>> {
        None
    }

    fn time_to_next_runs(&self, _n: usize) -> Option<Vec<Duration>> {
        None
    }
}
