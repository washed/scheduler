pub mod oneshot;
pub mod weekly;
pub mod interval;
mod tests;

use chrono::{DateTime, TimeZone};
use std::time::Duration;
use dyn_clone::{clone_trait_object, DynClone};

pub trait Trigger<Tz: TimeZone>: DynClone {
    fn next_runs(&self, _n: usize) -> Option<Vec<DateTime<Tz>>> {
        None
    }

    fn time_to_next_runs(&self, _n: usize) -> Option<Vec<Duration>> {
        None
    }
}

clone_trait_object!(<Tz: TimeZone> Trigger<Tz>);
