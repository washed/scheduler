use chrono::{DateTime, Datelike, Duration, DurationRound, Local, TimeZone, Utc, Date};

struct Weekly {
    pub weekdays: [bool; 7],
    pub time: Duration,
    // pub callback: dyn Fn(),
}

impl Weekly {
    pub fn next_runs<'a, Tz: TimeZone + 'a>(
        &'a self,
        n: usize,
        now: fn() -> DateTime<Tz>,
    ) -> impl Iterator<Item = DateTime<Tz>> + 'a {
        self.weekdays
            .iter()
            .cycle()
            .enumerate()
            .filter(|(_i, e)| **e)
            .skip(now().weekday().num_days_from_monday() as usize)
            .map(move |(i, _e)| {
                let now = now();
                let weekday = now.weekday();
                let weekday_offset = weekday.num_days_from_monday() as i64;
                let now_midnight = now.duration_round(Duration::days(1)).unwrap();
                now_midnight + Duration::days(i as i64 - weekday_offset) + self.time
            })
            .skip_while(move |dt| {
                let now = now();
                *dt < now
            })
            .take(n)
    }

    pub fn time_to_next_runs<'a, Tz: TimeZone + 'a>(
        &'a self,
        n: usize,
        now: fn() -> DateTime<Tz>,
    ) -> impl Iterator<Item = Duration> + 'a {
        self.next_runs(n, now).map(move |dt| {
            let now = now();
            dt - now
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hhmmss::Hhmmss;

    fn fake_now_utc() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap().into()
    }

    fn fake_now_local() -> DateTime<Local> {
        DateTime::parse_from_rfc3339("2023-01-01T00:00:00+01:00").unwrap().into()
    }

    #[test]
    fn it_works() {
        let weekly = Weekly {
            weekdays: [false, true, true, true, true, true, true],
            time: Duration::hours(12),
        };

        let ttnr: Vec<DateTime<Utc>> = weekly.next_runs(8, fake_now_utc).collect();
        let expected_ttnr: Vec<DateTime<Utc>> = [
            DateTime::parse_from_rfc3339("2023-01-01T12:00:00Z").unwrap().into(),
            // 2nd skipped because it's a monday
            DateTime::parse_from_rfc3339("2023-01-03T12:00:00Z").unwrap().into(),
            DateTime::parse_from_rfc3339("2023-01-04T12:00:00Z").unwrap().into(),
            DateTime::parse_from_rfc3339("2023-01-05T12:00:00Z").unwrap().into(),
            DateTime::parse_from_rfc3339("2023-01-06T12:00:00Z").unwrap().into(),
            DateTime::parse_from_rfc3339("2023-01-07T12:00:00Z").unwrap().into(),
            DateTime::parse_from_rfc3339("2023-01-08T12:00:00Z").unwrap().into(),
            // 9th skipped because it's a monday
            DateTime::parse_from_rfc3339("2023-01-10T12:00:00Z").unwrap().into(),
            DateTime::parse_from_rfc3339("2023-01-11T12:00:00Z").unwrap().into(),
        ].into();

        // TODO: I'd expect this test to pass, but for some reason we are skipping 2 days instead of one. Debug!

        assert_eq!(ttnr, expected_ttnr);
        for d in ttnr {
            let pretty = d;
            println!("Duration: {pretty}");
        }
    }
}
