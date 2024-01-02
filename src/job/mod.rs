use crate::trigger::{NowUtc, Trigger};

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, fmt};
use tokio::task::JoinSet;
use tokio::time::sleep;
use tracing::debug;

pub type Result<T> = std::result::Result<T, NoMoreRunsError>;

#[derive(Debug, Clone)]
pub struct NoMoreRunsError;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct TriggerCollection(pub BTreeSet<Box<dyn Trigger>>);

impl TriggerCollection {
    pub fn iter(&self) -> std::collections::btree_set::Iter<'_, Box<dyn Trigger>> {
        self.0.iter()
    }
}

#[macro_export]
macro_rules! triggerCollection {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = std::collections::BTreeSet::new();
            $(
                let boxed: std::boxed::Box<dyn $crate::trigger::Trigger + 'static> = std::boxed::Box::new($x);
                temp_set.insert(boxed);
            )*
            $crate::job::TriggerCollection(temp_set)
        }
    };
}

impl fmt::Display for NoMoreRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no more runs")
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Job {
    pub name: String,
    #[serde(skip)]
    callback: Option<fn()>,
    triggers: TriggerCollection,
}

#[cfg(not(test))]
impl NowUtc for Job {}

impl Job {
    pub fn new(name: String, callback: fn(), triggers: TriggerCollection) -> Self {
        Self {
            name,
            callback: Some(callback),
            triggers,
        }
    }

    pub fn next_run(triggers: &TriggerCollection) -> Option<DateTime<Utc>> {
        triggers
            .iter()
            .filter_map(|t: &Box<dyn Trigger>| {
                let next_run = t.next_runs(1);
                match next_run {
                    Some(next_run) => Some(next_run[0].to_owned()),
                    None => None,
                }
            })
            .sorted()
            .take(1)
            .next()
    }

    fn start_task(
        tasks: &mut JoinSet<Result<()>>,
        name: String,
        triggers: TriggerCollection,
        callback: fn(),
    ) {
        tasks.spawn(async move {
            loop {
                let next_run = Job::next_run(&triggers).ok_or(NoMoreRunsError)?;
                let sleep_time = next_run - Self::now_utc();
                debug!(name, at = { next_run.to_rfc3339() }, "in" = %sleep_time, "next run");
                let sleep_time = sleep_time
                    .clamp(ChronoDuration::zero(), ChronoDuration::max_value())
                    .to_std()
                    .unwrap();

                sleep(sleep_time).await;

                debug!(name, "triggered");
                callback();
            }
        });
    }

    pub fn run(job: Self, tasks: &mut JoinSet<Result<()>>) {
        Some(Job::start_task(
            tasks,
            job.name,
            job.triggers,
            job.callback.unwrap_or(|| {}),
        ));
    }
}
