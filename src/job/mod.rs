use crate::trigger::{NowUtc, TriggerSet};

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Debug;
use tokio::task::JoinSet;
use tokio::time::sleep;
use tracing::debug;

pub type Result<T> = std::result::Result<T, NoMoreRunsError>;

#[derive(Debug, Clone)]
pub struct NoMoreRunsError;

impl fmt::Display for NoMoreRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no more runs")
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Job {
    pub name: String,
    #[serde(skip)]
    callback: Option<fn()>,
    triggers: TriggerSet,
}

#[cfg(not(test))]
impl NowUtc for Job {}

impl Job {
    pub fn new(name: String, callback: Option<fn()>, triggers: TriggerSet) -> Self {
        Self {
            name,
            callback,
            triggers,
        }
    }

    pub fn next_run(triggers: &TriggerSet) -> Option<DateTime<Utc>> {
        triggers
            .iter()
            .filter_map(|t| {
                let next_run = t.next_runs(1);
                next_run.map(|next_run| next_run[0].to_owned())
            })
            .sorted()
            .take(1)
            .next()
    }

    fn start_task(
        tasks: &mut JoinSet<Result<()>>,
        name: String,
        triggers: TriggerSet,
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
        Job::start_task(tasks, job.name, job.triggers, job.callback.unwrap_or(|| {}));
    }
}
