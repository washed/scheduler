use std::iter::Iterator;

use chrono::{DateTime, Datelike, Duration, DurationRound, TimeZone};

pub trait Trigger<Tz: TimeZone> {
    fn next_runs(&self, _n: usize) -> Vec<DateTime<Tz>> {
        Vec::<DateTime<Tz>>::new()
    }

    fn time_to_next_runs(&self, _n: usize) -> Vec<Duration> {
        Vec::<Duration>::new()
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, Utc};

    fn fake_now_utc() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
            .unwrap()
            .into()
    }

    fn fake_now_local() -> DateTime<Local> {
        DateTime::parse_from_rfc3339("2023-01-01T01:00:00+01:00")
            .unwrap()
            .into()
    }

    fn fake_now_local_dst_spring() -> DateTime<Local> {
        DateTime::parse_from_rfc3339("2023-03-24T01:00:00+01:00")
            .unwrap()
            .into()
    }

    fn fake_now_local_dst_autumn() -> DateTime<Local> {
        DateTime::parse_from_rfc3339("2023-10-27T01:00:00+01:00")
            .unwrap()
            .into()
    }

    fn callback<Tz: TimeZone>(_dt: DateTime<Tz>) {
        println!("called!");
    }

    #[test]
    fn it_works_utc() {
        let weekly = Weekly {
            weekdays: [false, true, true, true, true, true, true],
            time: Duration::hours(12),
            callback: callback,
            now: fake_now_utc,
        };
        let ttnr: Vec<DateTime<Utc>> = weekly.next_runs(9);

        let expected_ttnr_utc: Vec<DateTime<Utc>> = [
            "2023-01-01T12:00:00Z",
            "2023-01-03T12:00:00Z",
            "2023-01-04T12:00:00Z",
            "2023-01-05T12:00:00Z",
            "2023-01-06T12:00:00Z",
            "2023-01-07T12:00:00Z",
            "2023-01-08T12:00:00Z",
            "2023-01-10T12:00:00Z",
            "2023-01-11T12:00:00Z",
        ]
        .iter()
        .map(|dts| DateTime::parse_from_rfc3339(dts).unwrap().into())
        .collect();
        assert_eq!(ttnr, expected_ttnr_utc);
    }

    #[test]
    fn it_works_local() {
        let weekly = Weekly {
            weekdays: [false, true, true, true, true, true, true],
            time: Duration::hours(12),
            callback: callback,
            now: fake_now_local,
        };
        let ttnr: Vec<DateTime<Local>> = weekly.next_runs(9);

        let expected_ttnr_local: Vec<DateTime<Local>> = [
            "2023-01-01T12:00:00+01:00",
            "2023-01-03T12:00:00+01:00",
            "2023-01-04T12:00:00+01:00",
            "2023-01-05T12:00:00+01:00",
            "2023-01-06T12:00:00+01:00",
            "2023-01-07T12:00:00+01:00",
            "2023-01-08T12:00:00+01:00",
            "2023-01-10T12:00:00+01:00",
            "2023-01-11T12:00:00+01:00",
        ]
        .iter()
        .map(|dts| DateTime::parse_from_rfc3339(dts).unwrap().into())
        .collect();
        assert_eq!(ttnr, expected_ttnr_local);
    }

    #[test]
    fn it_works_local_dst_change_spring() {
        let weekly = Weekly {
            weekdays: [false, true, true, true, true, true, true],
            time: Duration::hours(12),
            callback: callback,
            now: fake_now_local_dst_spring,
        };
        let ttnr: Vec<DateTime<Local>> = weekly.next_runs(9);

        let expected_ttnr_local: Vec<DateTime<Local>> = [
            "2023-03-24T12:00:00+01:00",
            "2023-03-25T12:00:00+01:00",
            "2023-03-26T12:00:00+02:00",
            "2023-03-28T12:00:00+02:00",
            "2023-03-29T12:00:00+02:00",
            "2023-03-30T12:00:00+02:00",
            "2023-03-31T12:00:00+02:00",
            "2023-04-01T12:00:00+02:00",
            "2023-04-02T12:00:00+02:00",
        ]
        .iter()
        .map(|dts| DateTime::parse_from_rfc3339(dts).unwrap().into())
        .collect();
        assert_eq!(ttnr, expected_ttnr_local);
    }

    #[test]
    fn it_works_local_dst_change_autumn() {
        let weekly = Weekly {
            weekdays: [false, true, true, true, true, true, true],
            time: Duration::hours(12),
            callback: callback,
            now: fake_now_local_dst_autumn,
        };
        let ttnr: Vec<DateTime<Local>> = weekly.next_runs(9);

        let expected_ttnr_local: Vec<DateTime<Local>> = [
            "2023-10-27T12:00:00+02:00",
            "2023-10-28T12:00:00+02:00",
            "2023-10-29T12:00:00+01:00",
            "2023-10-31T12:00:00+01:00",
            "2023-11-01T12:00:00+01:00",
            "2023-11-02T12:00:00+01:00",
            "2023-11-03T12:00:00+01:00",
            "2023-11-04T12:00:00+01:00",
            "2023-11-05T12:00:00+01:00",
        ]
        .iter()
        .map(|dts| DateTime::parse_from_rfc3339(dts).unwrap().into())
        .collect();
        assert_eq!(ttnr, expected_ttnr_local);
    }

    #[test]
    fn no_runs() {}
}
