#[cfg(test)]
mod tests {
    use crate::job::Job;
    use crate::scheduler::Scheduler;
    use crate::trigger::oneshot::Oneshot;
    use crate::trigger::weekly::Weekly;

    use chrono::{Duration, Utc};

    fn callback() {
        println!("yeayah");
    }

    #[tokio::test]
    async fn it_works_utc() {
        /*
        let weekly: Weekly<Utc> = Weekly::new(
            [false, true, true, true, true, true, true],
            Duration::hours(12).to_std().unwrap(),
            Utc::now,
        );
        */
        let oneshot = Oneshot::new(Utc::now() + std::time::Duration::from_secs(1), Utc::now);
        let job = Job::new(
            "test".to_string(),
            callback,
            vec![
                // Box::new(weekly),
                Box::new(oneshot),
            ],
        );
        let mut scheduler = Scheduler::<Utc>::new();
        scheduler.add_job(job);
        scheduler.run().await;
    }
}
