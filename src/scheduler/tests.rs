#[cfg(test)]
mod tests {
    use crate::job::Job;
    use crate::scheduler::Scheduler;
    use crate::trigger::oneshot::Oneshot;

    use chrono::{DateTime, Utc};

    fn callback() {
        println!("scheduler test callback");
    }

    #[tokio::test]
    async fn it_works_utc() {
        let oneshot = Oneshot::new(DateTime::<Utc>::default() + std::time::Duration::from_secs(1));
        let job = Job::new("test".to_string(), callback, vec![Box::new(oneshot)]);
        let mut scheduler = Scheduler::new();
        scheduler.add_job(job);
        scheduler.run().await;
    }
}
