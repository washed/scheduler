use super::{NowUtc, Trigger};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
pub struct Oneshot {
    datetime: DateTime<Utc>,
}

impl Oneshot {
    pub fn new(datetime: DateTime<Utc>) -> Self {
        Self { datetime }
    }
}

#[cfg(not(test))]
impl NowUtc for Oneshot {}

#[typetag::serde]
impl Trigger for Oneshot {
    fn next_runs(&self, _n: usize) -> Option<Vec<DateTime<Utc>>> {
        match self.datetime >= Self::now_utc() {
            true => Some(vec![self.datetime.clone()]),
            false => None,
        }
    }

    fn time_to_next_runs(&self, n: usize) -> Option<Vec<Duration>> {
        let next_runs = self.next_runs(n)?;
        Some(
            next_runs
                .into_iter()
                .map(move |dt| {
                    let now = Self::now_utc();
                    (dt.with_timezone(&Utc) - now).to_std().unwrap()
                })
                .collect(),
        )
    }

    fn hash(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
