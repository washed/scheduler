use chrono::TimeZone;
use tokio::task;

use crate::job::Job;
mod tests;

pub struct Scheduler<Tz: TimeZone> {
    jobs: Vec<Job<Tz>>,
    runner: Option<task::JoinHandle<()>>,
}

unsafe impl<Tz: TimeZone> Sync for Scheduler<Tz> {}
unsafe impl<Tz: TimeZone> Send for Scheduler<Tz> {}

impl<Tz: TimeZone + 'static> Scheduler<Tz> {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            runner: None,
        }
    }

    pub fn add_job(&mut self, job: Job<Tz>) {
        self.jobs.push(job);
    }

    pub async fn run(self)
    where
        <Tz as TimeZone>::Offset: Send,
    {
        for job in self.jobs {
            job.run().await;
        }
    }
}
