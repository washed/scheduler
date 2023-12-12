use std::iter::Iterator;

use chrono::{DateTime, Datelike, Duration, DurationRound, TimeZone};

use super::Trigger;

pub struct Weekly<Tz: TimeZone> {
    weekdays: [bool; 7],
    time: Duration,
    now: fn() -> DateTime<Tz>,

    pub callback: fn(dt: DateTime<Tz>),
}

impl<Tz: TimeZone> Weekly<Tz> {
    pub fn new(
        weekdays: [bool; 7],
        time: Duration,
        callback: fn(dt: DateTime<Tz>),
        now: fn() -> DateTime<Tz>,
    ) -> Self {
        Self {
            weekdays,
            time,
            callback,
            now,
        }
    }
}

impl<Tz: TimeZone> Trigger<Tz> for Weekly<Tz> {
    fn next_runs(&self, n: usize) -> Vec<DateTime<Tz>> {
        match self.weekdays.iter().all(|e| !e) {
            true => Vec::<DateTime<Tz>>::new(),
            false => self
                .weekdays
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
                        .duration_round(Duration::days(1))
                        .unwrap()
                        .naive_local();
                    let next_dt_naive =
                        (now_midnight + Duration::days(i as i64 - weekday_offset) + self.time)
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
        }
    }

    fn time_to_next_runs(&self, n: usize) -> Vec<Duration> {
        let next_runs = self.next_runs(n);
        next_runs
            .into_iter()
            .map(move |dt| {
                let now = (self.now)();
                dt - now
            })
            .collect()
    }
}
