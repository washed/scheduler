use chrono::{DateTime, TimeZone, Utc};

use crate::trigger::Trigger;

use itertools::Itertools;
use tokio::task::JoinSet;
use tokio::time::sleep;
use tracing::debug;

pub struct Job<Tz: TimeZone> {
    pub name: String,
    pub callback: fn(),
    triggers: Vec<Box<dyn Trigger<Tz> + Send + Sync>>,
}

impl<Tz: TimeZone + 'static> Job<Tz> {
    pub fn new(
        name: String,
        callback: fn(),
        triggers: Vec<Box<dyn Trigger<Tz> + Send + Sync>>,
    ) -> Self {
        Self {
            name,
            callback,
            triggers,
        }
    }

    pub fn next_run(triggers: &Vec<Box<dyn Trigger<Tz> + Send + Sync>>) -> Option<DateTime<Tz>> {
        triggers
            .iter()
            .filter_map(|t: &Box<dyn Trigger<Tz> + Send + Sync>| {
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
        tasks: &mut JoinSet<()>,
        name: String,
        triggers: Vec<Box<dyn Trigger<Tz> + Send + Sync>>,
        callback: fn(),
    ) where
        <Tz as TimeZone>::Offset: Send,
    {
        tasks.spawn(async move {
            let triggers = triggers.to_vec();
            loop {
                let next_run = Job::next_run(&triggers).unwrap(); // TODO: meh
                let next_run_utc = next_run.with_timezone(&Utc);
                let sleep_time = next_run_utc - Utc::now();

                let next_run_str = next_run.to_rfc3339();
                debug!("Next run of {name} at: {next_run_str}. Sleeping for {sleep_time}");

                let sleep_time_std = sleep_time.to_std().unwrap();
                sleep(sleep_time_std).await;
                debug!("executing {:?}", name);
                callback();
            }
        });
    }

    pub fn run(&mut self, tasks: &mut JoinSet<()>)
    where
        <Tz as TimeZone>::Offset: Send,
    {
        let triggers = self.triggers.as_slice().to_vec();
        Some(Job::start_task(
            tasks,
            self.name.to_owned(),
            triggers,
            self.callback,
        ));
    }
}
