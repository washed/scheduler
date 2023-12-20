use chrono::{DateTime, TimeZone, Utc};

use crate::trigger::Trigger;

use itertools::Itertools;
use tokio::task;
use tokio::time::sleep;

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Job<Tz: TimeZone> {
    pub name: String,
    pub callback: fn(),
    triggers: Arc<Mutex<Vec<Box<dyn Trigger<Tz> + Send + Sync>>>>,
    now: fn() -> DateTime<Tz>,
}

unsafe impl<Tz: TimeZone> Sync for Job<Tz> {}
unsafe impl<Tz: TimeZone> Send for Job<Tz> {}

impl<Tz: TimeZone + 'static> Job<Tz> {
    pub fn new(
        name: String,
        callback: fn(),
        triggers: Vec<Box<dyn Trigger<Tz> + Send + Sync>>,
        now: fn() -> DateTime<Tz>,
    ) -> Self {
        let triggers = Arc::new(Mutex::new(triggers));
        Self {
            name,
            callback,
            triggers,
            now,
        }
    }

    pub async fn next_run(&self) -> Option<DateTime<Tz>> {
        let triggers = self.triggers.lock().await;
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

    pub async fn run(self) -> task::JoinHandle<()>
    where
        <Tz as TimeZone>::Offset: Send,
    {
        task::spawn(async move {
            let name = self.name.to_owned();
            loop {
                let next_run = self.next_run().await.unwrap();
                let next_run_utc = next_run.with_timezone(&Utc);
                let sleep_time = next_run_utc - Utc::now();
                let sleep_time_str = sleep_time.num_seconds();

                let next_run_str = next_run.to_rfc3339();
                println!("Next run of {name} at: {next_run_str}. Sleeping for {sleep_time_str} s");

                let sleep_time_std = sleep_time.to_std().unwrap();
                sleep(sleep_time_std).await;
                println!("executing {:?}", name);
                (self.callback)();
            }
        })
    }
}
