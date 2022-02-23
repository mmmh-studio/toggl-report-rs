//! Data structures to store DetailedReports.
//!
//! For more information, see https://github.com/toggl/toggl_api_docs/blob/master/reports/detailed.md.

use crate::reports::{Currency, DurationWrapper, EarningWrapper};
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A structure to store DetailedReports.
#[derive(Debug, Deserialize)]
pub struct DetailedReport {
    pub total_grand: DurationWrapper,
    pub total_billable: DurationWrapper,
    pub total_count: u64,
    pub per_page: u64,
    pub total_currencies: Vec<Currency>,
    pub data: Vec<DataEntry>,
}

#[derive(Debug, Deserialize)]
pub struct DataEntry {
    pub id: u64,
    pub pid: Option<u64>,
    pub tid: Option<u64>,
    pub uid: Option<u64>,
    pub description: Option<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub dur: DurationWrapper,
    pub user: Option<String>,
    pub use_stop: bool,
    pub client: Option<String>,
    pub project: Option<String>,
    pub task: Option<String>,
    pub billable: EarningWrapper,
    pub is_billable: bool,
    pub cur: Option<String>,
    pub tags: Vec<String>,
}
