use chrono::{DateTime, Utc};
use std::sync::{Arc, RwLock};

pub fn dt_parse(dt_str: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(dt_str)
        .unwrap()
        .with_timezone(&Utc)
}

// borrowed with gratitude from https://blog.sentry.io/you-cant-rust-that/
#[derive(Default, Debug)]
pub struct Config {
    pub fake_start_time: DateTime<Utc>,
    pub start_time: Option<DateTime<Utc>>,
}

impl Config {
    pub fn current() -> Arc<Config> {
        CURRENT_CONFIG.with(|c| c.read().unwrap().clone())
    }
    pub fn make_current(self) {
        CURRENT_CONFIG.with(|c| *c.write().unwrap() = Arc::new(self))
    }

    pub fn get_fake_now() -> DateTime<Utc> {
        let config = Self::current();
        let now = Utc::now();
        if config.start_time.is_none() {
            Self {
                fake_start_time: config.fake_start_time,
                start_time: Some(now),
            }
            .make_current();
        }

        let config = Self::current();

        Self::current().fake_start_time + (now - config.start_time.unwrap())
    }
}

thread_local! {
    static CURRENT_CONFIG: RwLock<Arc<Config>> = RwLock::new(Default::default());
}

pub fn set_start_time(dt_str: &str) {
    Config {
        fake_start_time: dt_parse(dt_str),
        start_time: None,
    }
    .make_current();
}
