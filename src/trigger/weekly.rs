use super::Trigger;
use chrono::{DateTime, Datelike, DurationRound, TimeZone, Duration as ChronoDuration};
use std::time::Duration;

#[derive(Clone)]
pub struct Weekly<Tz: TimeZone> {
    weekdays: [bool; 7],
    time: Duration,
    now: fn() -> DateTime<Tz>,
}

impl<Tz: TimeZone> Weekly<Tz> {
    pub fn new(weekdays: [bool; 7], time: Duration, now: fn() -> DateTime<Tz>) -> Self {
        Self {
            weekdays,
            time,
            now,
        }
    }
}

impl<Tz: TimeZone> Trigger<Tz> for Weekly<Tz> {
    fn next_runs(&self, n: usize) -> Option<Vec<DateTime<Tz>>> {
        match self.weekdays.iter().all(|e| !e) {
            true => None,
            false => Some(
                self.weekdays
                    .iter()
                    .cycle()
                    .enumerate()
                    .skip((self.now)().weekday().num_days_from_monday() as usize)
                    .filter_map(move |(i, e)| {
                        let now = (self.now)();
                        let weekday = now.weekday();
                        let weekday_offset = weekday.num_days_from_monday() as i64;
                        let now_midnight = now
                            .clone()
                            .duration_round(ChronoDuration::days(1))
                            .unwrap()
                            .naive_local();
                        let next_dt_naive =
                            (now_midnight + ChronoDuration::days(i as i64 - weekday_offset) + self.time)
                                .and_local_timezone(now.timezone());
                        match next_dt_naive {
                            chrono::LocalResult::None => None,
                            chrono::LocalResult::Ambiguous(_, _) => None,
                            chrono::LocalResult::Single(res) => Some((*e, res)),
                        }
                    })
                    .skip_while(move |(_e, dt)| *dt < (self.now)())
                    .filter_map(|(e, dt)| match e {
                        true => Some(dt),
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
                    let now = (self.now)();
                    (dt - now).to_std().unwrap()
                })
                .collect(),
        )
    }
}
