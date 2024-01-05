use crate::tests::fake_time::{dt_parse, set_start_time};
use crate::tests::DEFAULT_UTC;

use crate::job::Job;
use crate::trigger::{Interval, Oneshot, Weekly};
use crate::triggerSet;
use chrono::{DateTime, Utc};
use chrono_tz::UTC;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::task::JoinSet;

fn callback(context: &Value) {
    println!("test job callback");
}

#[tokio::test]
async fn test_job_run() {
    set_start_time(DEFAULT_UTC);
    let oneshot = Oneshot::new(dt_parse(DEFAULT_UTC) + std::time::Duration::from_secs(1));
    let tc = triggerSet![oneshot];
    let cb_context = json!({
        "bar": 42,
        "baz": "weeeeh"
    });
    let job = Job::new("test".to_string(), Some(callback), cb_context, tc);

    fn callback(context: &Value) {
        assert_eq!(
            *context,
            json!({
                "bar": 42,
                "baz": "weeeeh"
            })
        );
    }

    let mut join_set = JoinSet::new();

    Job::run(job, &mut join_set);

    join_set.join_next().await;
}

#[tokio::test]
async fn test_trigger_collection() {
    let oneshot = Oneshot::new(DateTime::<Utc>::default() + std::time::Duration::from_secs(1));
    let interval = Interval::new(std::time::Duration::from_secs(1));
    let weekly = Weekly::new(
        [true, true, true, true, false, false, false],
        Duration::from_secs(60),
        UTC,
    );
    let tc = triggerSet![oneshot, interval, weekly];
    let tc_json = serde_json::to_string(&tc).unwrap();
    println!("{:#?}", tc_json);
}

#[tokio::test]
async fn test_job_serialize() {
    let oneshot = Oneshot::new(DateTime::<Utc>::default() + std::time::Duration::from_secs(1));
    let interval = Interval::new(std::time::Duration::from_secs(1));
    let weekly = Weekly::new(
        [true, true, true, true, false, false, false],
        Duration::from_secs(60),
        UTC,
    );
    let job = Job::new(
        "test".to_string(),
        Some(callback),
        Value::Null,
        triggerSet![oneshot, interval, weekly],
    );

    let expected_job_json = r#"{"name":"test","callback_context":null,"triggers":[{"type":"Oneshot","datetime":"1970-01-01T00:00:01Z"},{"type":"Interval","interval":{"secs":1,"nanos":0},"last_run":null},{"type":"Weekly","weekdays":[true,true,true,true,false,false,false],"time":{"secs":60,"nanos":0},"tz":"UTC"}]}"#;

    let job_json = serde_json::to_string(&job).unwrap();
    assert_eq!(expected_job_json, job_json);
}

#[tokio::test]
async fn test_job_deserialize() {
    let oneshot = Oneshot::new(DateTime::<Utc>::default() + std::time::Duration::from_secs(1));
    let interval = Interval::new(std::time::Duration::from_secs(1));
    let weekly = Weekly::new(
        [true, true, true, true, false, false, false],
        Duration::from_secs(60),
        UTC,
    );
    let expected_job = Job::new(
        "test".to_string(),
        None,
        Value::Null,
        triggerSet![oneshot, interval, weekly],
    );

    let job_json = r#"{"name":"test","callback_context":null,"triggers":[{"type":"Oneshot","datetime":"1970-01-01T00:00:01Z"},{"type":"Interval","interval":{"secs":1,"nanos":0},"last_run":null},{"type":"Weekly","weekdays":[true,true,true,true,false,false,false],"time":{"secs":60,"nanos":0},"tz":"UTC"}]}"#;
    let job: Job = serde_json::from_str(job_json).unwrap();

    assert_eq!(expected_job, job);
}
