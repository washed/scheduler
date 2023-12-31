mod fake_time;
mod job;
mod scheduler;
mod trigger;

#[cfg(test)]
mod tests {
    use crate::job::Job;
    use crate::tests::fake_time::Config;
    use crate::trigger::interval::Interval;
    use crate::trigger::oneshot::Oneshot;
    use crate::trigger::weekly::Weekly;
    use crate::trigger::NowUtc;

    use chrono::{DateTime, Utc};

    impl NowUtc for Oneshot {
        fn now_utc() -> DateTime<Utc> {
            Config::get_fake_now()
        }
    }

    impl NowUtc for Interval {
        fn now_utc() -> DateTime<Utc> {
            Config::get_fake_now()
        }
    }

    impl NowUtc for Weekly {
        fn now_utc() -> DateTime<Utc> {
            Config::get_fake_now()
        }
    }

    impl NowUtc for Job {
        fn now_utc() -> DateTime<Utc> {
            Config::get_fake_now()
        }
    }

    pub const DEFAULT_UTC: &str = "2023-01-01T00:00:00Z";
    pub const DST_SPRING_LOCAL: &str = "2023-03-24T01:00:00+01:00";
    pub const DST_AUTUMN_LOCAL: &str = "2023-10-27T01:00:00+01:00";
}
