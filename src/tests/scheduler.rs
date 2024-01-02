#[cfg(test)]
mod tests {
    use crate::tests::fake_time::{dt_parse, set_start_time};
    use crate::tests::tests::DEFAULT_UTC;

    use crate::job::Job;
    use crate::scheduler::Scheduler;
    use crate::trigger::Oneshot;
    use crate::triggerCollection;

    fn callback() {
        println!("test scheduler callback");
    }

    #[tokio::test]
    async fn it_works_utc() {
        set_start_time(DEFAULT_UTC);
        let test_time = dt_parse(DEFAULT_UTC);
        let oneshot = Oneshot::new(test_time + std::time::Duration::from_secs(1));
        let job = Job::new("test".to_string(), callback, triggerCollection![oneshot]);
        let mut scheduler = Scheduler::new();
        scheduler.add_job(job);
        scheduler.run().await;
    }
}
