#[cfg(test)]
mod tests {
    use crate::trigger::oneshot::Oneshot;
    use crate::trigger::weekly::Weekly;
    use crate::trigger::Trigger;
    use chrono::{DateTime, Local, Duration, Utc};

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

    #[test]
    fn it_works_utc() {
        let weekly = Weekly::new(
            [false, true, true, true, true, true, true],
            Duration::hours(12).to_std().unwrap(),
            fake_now_utc,
        );
        let ttnr: Vec<DateTime<Utc>> = weekly.next_runs(9).unwrap();

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
        let weekly = Weekly::new(
            [false, true, true, true, true, true, true],
            Duration::hours(12).to_std().unwrap(),
            fake_now_local,
        );
        let ttnr: Vec<DateTime<Local>> = weekly.next_runs(9).unwrap();

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
        let weekly = Weekly::new(
            [false, true, true, true, true, true, true],
            Duration::hours(12).to_std().unwrap(),
            fake_now_local_dst_spring,
        );
        let ttnr: Vec<DateTime<Local>> = weekly.next_runs(9).unwrap();

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
        let weekly = Weekly::new(
            [false, true, true, true, true, true, true],
            Duration::hours(12).to_std().unwrap(),
            fake_now_local_dst_autumn,
        );
        let ttnr: Vec<DateTime<Local>> = weekly.next_runs(9).unwrap();

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
    fn no_runs() {
        let weekly = Weekly::new(
            [false, false, false, false, false, false, false],
            Duration::hours(12).to_std().unwrap(),
            fake_now_utc,
        );
        let ttnr = weekly.next_runs(9);

        assert_eq!(ttnr, None);
    }

    #[test]
    fn oneshot_future() {
        let run_time = fake_now_utc() + Duration::hours(1);
        let oneshot = Oneshot::new(run_time, fake_now_utc);
        let next_runs: Vec<DateTime<Utc>> = oneshot.next_runs(1).unwrap();

        assert_eq!(next_runs.len(), 1);
        assert_eq!(next_runs[0], run_time);
    }

    #[test]
    fn oneshot_past() {
        let run_time = fake_now_utc() - Duration::hours(1);
        let oneshot = Oneshot::new(run_time, fake_now_utc);
        let next_runs = oneshot.next_runs(1);

        assert_eq!(next_runs, None);
    }
}
