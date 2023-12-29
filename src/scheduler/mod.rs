use crate::job::{Job, Result};
use tokio::task::JoinSet;
mod tests;
use tracing::{error, info, warn};

pub struct Scheduler {
    jobs: Vec<Job>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn add_job(&mut self, job: Job) {
        self.jobs.push(job);
    }

    pub async fn run(self) {
        let mut tasks = JoinSet::<Result<()>>::new();
        for mut job in self.jobs {
            job.run(&mut tasks);
        }

        while tasks.len() > 0 {
            match tasks.join_next().await {
                Some(result) => match result {
                    Ok(task_return) => match task_return {
                        Ok(_) => {
                            info!("task finished")
                        }
                        Err(error) => warn!(%error, "task returned with error"),
                    },
                    Err(error) => error!(%error, "task panicked"),
                },
                None => {
                    info!("no more tasks to run, shutting down")
                }
            }
        }
    }
}
