pub mod interval;
pub mod oneshot;
pub mod trigger_set;
pub mod weekly;

use chrono::{DateTime, Utc};
use std::time::Duration;

#[typetag::serde(tag = "type")]
pub trait Trigger: std::fmt::Debug
where
    Self: Send,
{
    fn next_runs(&self, _n: usize) -> Option<Vec<DateTime<Utc>>> {
        None
    }

    fn time_to_next_runs(&self, _n: usize) -> Option<Vec<Duration>> {
        None
    }

    fn hash(&self) -> String;
}

impl PartialEq for dyn Trigger {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}

impl Eq for dyn Trigger {}

impl PartialOrd for dyn Trigger {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.hash().cmp(&other.hash()))
    }
}

impl Ord for dyn Trigger {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hash().cmp(&other.hash())
    }
}

pub trait NowUtc {
    fn now_utc() -> DateTime<Utc> {
        Utc::now()
    }
}

pub use self::{interval::Interval, oneshot::Oneshot, trigger_set::TriggerSet, weekly::Weekly};
