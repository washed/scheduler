#[cfg(test)]
mod tests {
    use crate::tests::fake_time::{dt_parse, set_start_time};
    use crate::tests::tests::DEFAULT_UTC;

    use crate::job::Job;
    use crate::trigger::Oneshot;
    use chrono::{DateTime, Utc};
    use tokio::task::JoinSet;

    fn callback() {
        println!("test job callback");
    }

    #[tokio::test]
    async fn test_job_run() {
        set_start_time(DEFAULT_UTC);
        let oneshot = Oneshot::new(dt_parse(DEFAULT_UTC) + std::time::Duration::from_secs(1));
        let mut job = Job::new("test".to_string(), callback, vec![Box::new(oneshot)]);

        let mut join_set = JoinSet::new();

        job.run(&mut join_set);
        match join_set.join_next().await {
            Some(_) => {}
            None => {}
        }
    }

    #[tokio::test]
    async fn test_job_serialize() {
        let oneshot = Oneshot::new(DateTime::<Utc>::default() + std::time::Duration::from_secs(1));
        let job = Job::new("test".to_string(), callback, vec![Box::new(oneshot)]);

        let foo = serde_json::to_string(&job).unwrap();
        println!("{foo}");
    }
}
