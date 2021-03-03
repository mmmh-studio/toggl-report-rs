//! Data structures to store WeeklyReports.
//!
//! For more information, see https://github.com/toggl/toggl_api_docs/blob/master/reports/weekly.md.

use crate::reports::{
    EarningTotals, TimeTotals, Report,
    ProjectTitle, UserTitle,
};
use serde::Deserialize;

/// A data structure to store WeeklyReports.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum WeeklyReport {
    ProjectTimeWeeklyReport (
        Report<ProjectData<TimeTotals>>
    ),
    UserTimeWeeklyReport (
        Report<UserData<TimeTotals>>
    ),
    ProjectEarningWeeklyReport (
        Report<ProjectData<EarningTotals>>
    ),
    UserEarningWeeklyReport (
        Report<UserData<EarningTotals>>
    )
}

#[derive(Debug, Deserialize)]
pub struct ProjectData<Totals> {
    pub title: ProjectTitle,
    pub pid: Option<u64>,
    pub totals: Totals,
    pub details: Vec<ProjectDetail<Totals>>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectDetail<Totals> {
    pub uid: Option<u64>,
    pub title: UserTitle,
    pub totals: Totals,
}

#[derive(Debug, Deserialize)]
pub struct UserData<Totals> {
    pub title: UserTitle,
    pub uid: Option<u64>,
    pub totals: Totals,
    pub details: Vec<UserDetail<Totals>>,
}

#[derive(Debug, Deserialize)]
pub struct UserDetail<Totals> {
    pub pid: Option<u64>,
    pub title: ProjectTitle,
    pub totals: Totals,
}

