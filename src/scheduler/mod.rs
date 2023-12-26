use crate::job::Job;
use chrono::TimeZone;
use tokio::task::JoinSet;
mod tests;
use tracing::{info, warn};

pub struct Scheduler<Tz: TimeZone> {
    jobs: Vec<Job<Tz>>,
}

impl<Tz: TimeZone + 'static> Scheduler<Tz> {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn add_job(&mut self, job: Job<Tz>) {
        self.jobs.push(job);
    }

    pub async fn run(self)
    where
        <Tz as TimeZone>::Offset: Send,
    {
        let mut tasks = JoinSet::<()>::new();
        for mut job in self.jobs {
            job.run(&mut tasks);
        }

        while tasks.len() > 0 {
            match tasks.join_next().await {
                Some(_task) => {
                    warn!("task ended unexpectedly")
                }
                None => {
                    info!("no more tasks to run, shutting down")
                }
            }
        }
    }
}
