pub mod interval;
pub mod oneshot;
mod tests;
pub mod weekly;

use chrono::{DateTime, Utc};
use dyn_clone::{clone_trait_object, DynClone};
use std::time::Duration;

pub trait Trigger: DynClone {
    fn next_runs(&self, _n: usize) -> Option<Vec<DateTime<Utc>>> {
        None
    }

    fn time_to_next_runs(&self, _n: usize) -> Option<Vec<Duration>> {
        None
    }
}

clone_trait_object!(Trigger);

pub trait NowUtc {
    fn now_utc() -> DateTime<Utc> {
        Utc::now()
    }
}
