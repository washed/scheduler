use crate::trigger::Trigger;

use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::Serialize;
use std::fmt;
use tokio::task::JoinSet;
use tokio::time::sleep;
use tracing::debug;

pub type Result<T> = std::result::Result<T, NoMoreRunsError>;

#[derive(Debug, Clone, Serialize)]
pub struct NoMoreRunsError;

impl fmt::Display for NoMoreRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no more runs")
    }
}

pub struct Job {
    pub name: String,
    pub callback: fn(),
    triggers: Vec<Box<dyn Trigger + Send + Sync>>,
}

impl Job {
    pub fn new(
        name: String,
        callback: fn(),
        triggers: Vec<Box<dyn Trigger + Send + Sync>>,
    ) -> Self {
        Self {
            name,
            callback,
            triggers,
        }
    }

    pub fn next_run(triggers: &Vec<Box<dyn Trigger + Send + Sync>>) -> Option<DateTime<Utc>> {
        triggers
            .iter()
            .filter_map(|t: &Box<dyn Trigger + Send + Sync>| {
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
        triggers: Vec<Box<dyn Trigger + Send + Sync>>,
        callback: fn(),
    ) {
        tasks.spawn(async move {
            let triggers = triggers.to_vec();
            loop {
                let next_run = Job::next_run(&triggers).ok_or(NoMoreRunsError)?;
                let sleep_time = next_run - Utc::now();
                // TODO: handle being late somehow
                debug!(name, at = { next_run.to_rfc3339() }, "in" = %sleep_time, "next run");
                let sleep_time = sleep_time.to_std().unwrap();

                sleep(sleep_time).await;

                debug!(name, "triggered");
                callback();
            }
        });
    }

    pub fn run(&mut self, tasks: &mut JoinSet<Result<()>>) {
        let triggers = self.triggers.as_slice().to_vec();
        Some(Job::start_task(
            tasks,
            self.name.to_owned(),
            triggers,
            self.callback,
        ));
    }
}
