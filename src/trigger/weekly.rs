use super::{NowUtc, Trigger};
use chrono::{DateTime, Datelike, Duration as ChronoDuration, DurationRound, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Tz(chrono_tz::Tz);

impl PartialOrd for Tz {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.name().partial_cmp(other.0.name())
    }
}

impl Ord for Tz {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.name().cmp(other.0.name())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Weekly {
    weekdays: [bool; 7],
    time: Duration,
    tz: Tz,
}

impl Weekly {
    pub fn new(weekdays: [bool; 7], time: Duration, tz: chrono_tz::Tz) -> Self {
        Self {
            weekdays,
            time,
            tz: Tz(tz),
        }
    }
}

#[cfg(not(test))]
impl NowUtc for Weekly {}

#[typetag::serde]
impl Trigger for Weekly {
    fn next_runs(&self, n: usize) -> Option<Vec<DateTime<Utc>>> {
        match self.weekdays.iter().all(|e| !e) {
            true => None,
            false => Some(
                self.weekdays
                    .iter()
                    .cycle()
                    .enumerate()
                    .skip(Self::now_utc().weekday().num_days_from_monday() as usize)
                    .filter_map(move |(i, e)| {
                        let now = Self::now_utc().with_timezone(&self.tz.0);
                        let weekday = now.weekday();
                        let weekday_offset = weekday.num_days_from_monday() as i64;
                        let now_midnight = now
                            .clone()
                            .duration_round(ChronoDuration::days(1))
                            .unwrap()
                            .naive_local();
                        let next_dt_naive = (now_midnight
                            + ChronoDuration::days(i as i64 - weekday_offset)
                            + self.time)
                            .and_local_timezone(self.tz.0);
                        match next_dt_naive {
                            chrono::LocalResult::None => None,
                            chrono::LocalResult::Ambiguous(_, _) => None,
                            chrono::LocalResult::Single(res) => Some((*e, res)),
                        }
                    })
                    .skip_while(move |(_e, dt)| *dt < Self::now_utc())
                    .filter_map(|(e, dt)| match e {
                        true => Some(dt.with_timezone(&Utc)),
                        false => None,
                    })
                    .take(n)
                    .collect(),
            ),
        }
    }

    fn time_to_next_runs(&self, n: usize) -> Option<Vec<Duration>> {
        let next_runs = self.next_runs(n)?;
        Some(
            next_runs
                .into_iter()
                .map(move |dt| {
                    let now = Self::now_utc();
                    (dt - now).to_std().unwrap()
                })
                .collect(),
        )
    }

    fn hash(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
