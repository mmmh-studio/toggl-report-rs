//! Data structures to store DetailedReports.
//!
//! For more information, see https://github.com/toggl/toggl_api_docs/blob/master/reports/detailed.md.

use chrono::{DateTime, Utc};
use crate::reports::{EarningWrapper, Currency, DurationWrapper};
use serde::Deserialize;


/// A structure to store DetailedReports.
#[derive(Debug, Deserialize)]
pub struct DetailedReport {
    total_grand: DurationWrapper,
    total_billable: DurationWrapper,
    total_count: u64,
    per_page: u64,
    total_currencies: Vec<Currency>,
    data: Vec<DataEntry>,
}

#[derive(Debug, Deserialize)]
pub struct DataEntry {
    id: u64,
    pid: Option<u64>,
    tid: Option<u64>,
    uid: Option<u64>,
    description: Option<String>,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    dur: DurationWrapper,
    user: Option<String>,
    use_stop: bool,
    client: Option<String>,
    project: Option<String>,
    task: Option<String>,
    billable: EarningWrapper,
    is_billable: bool,
    cur: Option<String>,
    tags: Vec<String>,
}