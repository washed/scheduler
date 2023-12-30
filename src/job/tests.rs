#[cfg(test)]
mod tests {
    use crate::job::Job;
    use crate::trigger::oneshot::Oneshot;
    use serde_json;
    use tokio::task::JoinSet;

    use chrono::{DateTime, Utc};

    fn callback() {
        println!("yeayah");
    }

    #[tokio::test]
    async fn test_job_run() {
        let oneshot = Oneshot::new(DateTime::<Utc>::default() + std::time::Duration::from_secs(1));
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
