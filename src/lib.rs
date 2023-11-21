use chrono::{Date, DateTime, Datelike, Duration, DurationRound, Local, TimeZone, Utc};

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
    ) -> Option<impl Iterator<Item = DateTime<Tz>> + 'a> {
        match self.weekdays.iter().all(|e| !e) {
            true => None,
            false => Some(
                self.weekdays
                    .iter()
                    .cycle()
                    .enumerate()
                    .skip(now().weekday().num_days_from_monday() as usize)
                    .map(move |(i, e)| {
                        let now = now();
                        let weekday = now.weekday();
                        let weekday_offset = weekday.num_days_from_monday() as i64;
                        let now_midnight = now.duration_round(Duration::days(1)).unwrap();
                        (
                            *e,
                            now_midnight + Duration::days(i as i64 - weekday_offset) + self.time,
                        )
                    })
                    .skip_while(move |(_e, dt)| {
                        let now = now();
                        *dt < now
                    })
                    .filter_map(|(e, dt)| match e {
                        true => Some(dt),
                        false => None,
                    })
                    .take(n),
            ),
        }
    }

    pub fn time_to_next_runs<'a, Tz: TimeZone + 'a>(
        &'a self,
        n: usize,
        now: fn() -> DateTime<Tz>,
    ) -> Option<impl Iterator<Item = Duration> + 'a> {
        let next_runs = self.next_runs(n, now)?;
        Some(next_runs.map(move |dt| {
            let now = now();
            dt - now
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hhmmss::Hhmmss;

    fn fake_now_utc() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
            .unwrap()
            .into()
    }

    fn fake_now_local() -> DateTime<Local> {
        DateTime::parse_from_rfc3339("2023-01-01T00:00:00+01:00")
            .unwrap()
            .into()
    }

    #[test]
    fn it_works() {
        let weekly = Weekly {
            weekdays: [false, true, true, true, true, true, true],
            // weekdays: [false, false, false, false, false, false, false],
            time: Duration::hours(12),
        };

        let ttnr: Vec<DateTime<Utc>> = weekly.next_runs(9, fake_now_utc).unwrap().collect();
        let expected_ttnr: Vec<DateTime<Utc>> = [
            DateTime::parse_from_rfc3339("2023-01-01T12:00:00Z")
                .unwrap()
                .into(),
            // 2nd skipped because it's a monday
            DateTime::parse_from_rfc3339("2023-01-03T12:00:00Z")
                .unwrap()
                .into(),
            DateTime::parse_from_rfc3339("2023-01-04T12:00:00Z")
                .unwrap()
                .into(),
            DateTime::parse_from_rfc3339("2023-01-05T12:00:00Z")
                .unwrap()
                .into(),
            DateTime::parse_from_rfc3339("2023-01-06T12:00:00Z")
                .unwrap()
                .into(),
            DateTime::parse_from_rfc3339("2023-01-07T12:00:00Z")
                .unwrap()
                .into(),
            DateTime::parse_from_rfc3339("2023-01-08T12:00:00Z")
                .unwrap()
                .into(),
            // 9th skipped because it's a monday
            DateTime::parse_from_rfc3339("2023-01-10T12:00:00Z")
                .unwrap()
                .into(),
            DateTime::parse_from_rfc3339("2023-01-11T12:00:00Z")
                .unwrap()
                .into(),
        ]
        .into();

        assert_eq!(ttnr, expected_ttnr);
        for d in ttnr {
            let pretty = d;
            println!("Duration: {pretty}");
        }
    }
}
