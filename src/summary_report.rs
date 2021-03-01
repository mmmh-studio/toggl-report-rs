//! Data structures to store SummaryReports.
//!
//! For more information, see https://github.com/toggl/toggl_api_docs/blob/master/reports/summary.md.

use serde::Deserialize;
use crate::reports::{
    EarningWrapper, Currency, DurationWrapper, Report, Title
};

/// A structure to store SummaryReports.
pub type SummaryReport = Report<SummaryData>;

#[derive(Debug, Deserialize)]
pub struct SummaryData {
    pub id: Option<u64>,
    pub title: Title,
    pub time: DurationWrapper,
    pub total_currencies: Vec<Currency>,
    pub items: Vec<SummaryItem>
}

#[derive(Debug, Deserialize)]
pub struct SummaryItem {
    pub title: Title,
    pub time: DurationWrapper,
    pub cur: Option<String>,
    pub sum: EarningWrapper,
    pub rate: EarningWrapper,
}