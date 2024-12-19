use std::str::FromStr;

use apalis::{layers::retry::RetryPolicy, prelude::*};
use apalis_cron::{CronStream, Schedule};
use chrono::{DateTime, Utc};
use tracing::warn;

#[derive(Default, Debug, Clone)]
struct Reminder(DateTime<Utc>);
impl From<DateTime<Utc>> for Reminder {
    fn from(t: DateTime<Utc>) -> Self {
        Reminder(t)
    }
}

async fn every_second(job: Reminder, job_id: TaskId) {
    // TODO: Do something every hour
    warn!("job {:?} with id {job_id:?} has not been executed", job.0);
}

pub fn apalis_monitor() -> Monitor {
    let schedule = Schedule::from_str("1/1 * * * * *").expect("Could not parse Schedule");
    let cron = CronStream::new(schedule);
    Monitor::new().register(
        WorkerBuilder::new("cron-scheduler")
            .enable_tracing()
            .catch_panic()
            .concurrency(3)
            .retry(RetryPolicy::retries(3))
            .backend(cron)
            .build_fn(every_second),
    )
}
