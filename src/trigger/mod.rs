pub mod interval;
pub mod oneshot;
mod tests;
pub mod weekly;

use chrono::{DateTime, TimeZone};
use dyn_clone::{clone_trait_object, DynClone};
use std::time::Duration;

pub trait Trigger<Tz: TimeZone>: DynClone {
    fn next_runs(&self, _n: usize) -> Option<Vec<DateTime<Tz>>> {
        None
    }

    fn time_to_next_runs(&self, _n: usize) -> Option<Vec<Duration>> {
        None
    }
}

clone_trait_object!(<Tz: TimeZone> Trigger<Tz>);
