use crate::tests::fake_time::{dt_parse, set_start_time};
use crate::tests::{DEFAULT_UTC, DST_AUTUMN_LOCAL, DST_SPRING_LOCAL};

use crate::trigger::{Interval, Oneshot, Trigger, Weekly};
use chrono::{DateTime, Duration, Local, Utc};
use chrono_tz::{Europe::Berlin, UTC};

#[test]
fn it_works_utc() {
    set_start_time(DEFAULT_UTC);
    let weekly = Weekly::new(
        [false, true, true, true, true, true, true],
        Duration::hours(12).to_std().unwrap(),
        UTC,
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
    set_start_time(DEFAULT_UTC);
    let weekly = Weekly::new(
        [false, true, true, true, true, true, true],
        Duration::hours(12).to_std().unwrap(),
        Berlin,
    );
    let ttnr: Vec<DateTime<Utc>> = weekly.next_runs(9).unwrap();

    let expected_ttnr_local: Vec<DateTime<Utc>> = [
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
    .map(|dts| {
        DateTime::parse_from_rfc3339(dts)
            .unwrap()
            .with_timezone(&Utc)
    })
    .collect();
    assert_eq!(ttnr, expected_ttnr_local);
}

#[test]
fn it_works_local_dst_change_spring() {
    set_start_time(DST_SPRING_LOCAL);
    let weekly = Weekly::new(
        [false, true, true, true, true, true, true],
        Duration::hours(12).to_std().unwrap(),
        Berlin,
    );
    let ttnr: Vec<DateTime<Utc>> = weekly.next_runs(9).unwrap();

    let expected_ttnr_local: Vec<DateTime<Utc>> = [
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
    .map(|dts| {
        DateTime::parse_from_rfc3339(dts)
            .unwrap()
            .with_timezone(&Utc)
    })
    .collect();
    assert_eq!(ttnr, expected_ttnr_local);
}

#[test]
fn it_works_local_dst_change_autumn() {
    set_start_time(DST_AUTUMN_LOCAL);
    let weekly = Weekly::new(
        [false, true, true, true, true, true, true],
        Duration::hours(12).to_std().unwrap(),
        Berlin,
    );
    let ttnr: Vec<DateTime<Utc>> = weekly.next_runs(9).unwrap();

    let expected_ttnr_local: Vec<DateTime<Utc>> = [
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
    .map(|dts| {
        DateTime::parse_from_rfc3339(dts)
            .unwrap()
            .with_timezone(&Utc)
    })
    .collect();
    assert_eq!(ttnr, expected_ttnr_local);
}

#[test]
fn no_runs() {
    set_start_time(DEFAULT_UTC);
    let weekly = Weekly::new(
        [false, false, false, false, false, false, false],
        Duration::hours(12).to_std().unwrap(),
        UTC,
    );
    let ttnr = weekly.next_runs(9);

    assert_eq!(ttnr, None);
}

#[test]
fn oneshot_future() {
    set_start_time(DEFAULT_UTC);
    let run_time = dt_parse(DEFAULT_UTC) + Duration::hours(1);
    let oneshot = Oneshot::new(run_time);
    let next_runs: Vec<DateTime<Utc>> = oneshot.next_runs(1).unwrap();

    assert_eq!(next_runs.len(), 1);
    assert_eq!(next_runs[0], run_time);
}

#[test]
fn oneshot_past() {
    set_start_time(DEFAULT_UTC);
    let run_time = dt_parse(DEFAULT_UTC) - Duration::hours(1);
    let oneshot = Oneshot::new(run_time);
    let next_runs = oneshot.next_runs(1);

    assert_eq!(next_runs, None);
}

#[test]
fn interval() {
    set_start_time(DEFAULT_UTC);
    let interval = Interval::new(Duration::seconds(1).to_std().unwrap());
    let next_runs = interval.next_runs(5).unwrap();

    let expected_next_runs: Vec<DateTime<Local>> = [
        "2023-01-01T00:00:01Z",
        "2023-01-01T00:00:02Z",
        "2023-01-01T00:00:03Z",
        "2023-01-01T00:00:04Z",
        "2023-01-01T00:00:05Z",
    ]
    .iter()
    .map(|dts| DateTime::parse_from_rfc3339(dts).unwrap().into())
    .collect();

    assert_eq!(next_runs, expected_next_runs)
}

#[test]
fn interval_from_json() {
    set_start_time(DEFAULT_UTC);
    let interval = Interval::new(Duration::seconds(1).to_std().unwrap());
    let _j = serde_json::to_string(&interval).unwrap();

    let _interval: Interval = serde_json::from_str(
        r#"{
            "interval": {"secs":1,"nanos":0},
            "last_run": null
        }"#,
    )
    .unwrap();
}
