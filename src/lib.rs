//! A library to call Toggl Reports API v2 in Rust.
//!
//! # examples
//!
//! ```no_run
//! use toggl_reports::{Toggl, query::Query, query::GroupingKind, query::OrderKind};
//! use chrono::Utc;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_token: &str = "00000000000000000000000000000000";  // your api token
//!     let workspace_id: u64 = 264;                               // your workspace id
//!
//!     let toggl = Toggl::new(api_token);
//!     let query = Query::new(workspace_id)
//!         .grouping(GroupingKind::Projects)
//!         .order_field(OrderKind::Description)
//!         .since(Utc::today());
//!
//!     let res = toggl.get_summary_report(&query).await.unwrap();
//!     println!("TOTAL: {}s", res.total_grand.num_seconds());
//! }
//! ```

#[macro_use]
extern crate enum_display_derive;

pub mod weekly_report;
pub mod detailed_report;
pub mod summary_report;
pub mod reports;
pub mod query;

use reqwest::Client;
use anyhow::Result;
use query::Query;
pub use weekly_report::WeeklyReport;
pub use detailed_report::DetailedReport;
pub use summary_report::SummaryReport;

const DEFAULT_UA: &'static str = "komori-n<ikamat.kmr@gmail.com>";

/// A structure to wrap Toggl Reports API v2.
#[derive(Debug)]
pub struct Toggl {
    api_token: String,
    user_agent: String,
}

impl Toggl {
    pub fn new(api_token: &str) -> Self {
        Self {
            api_token: api_token.to_owned(),
            user_agent: DEFAULT_UA.to_owned(),
        }
    }

    pub fn with_user_agent(api_token: &str, user_agent: &str) -> Self {
        Self {
            api_token: api_token.to_owned(),
            user_agent: user_agent.to_owned(),
        }
    }

    async fn rest_get(&self, endpoint: &str, query: &Query) -> Result<String> {
        let client = Client::new();
        let mut query_vec = query.to_vec();
        query_vec.push(("user_agent", self.user_agent.clone()));

        let req = client
            .get(endpoint)
            .query(&query_vec)
            .basic_auth(&self.api_token, Some("api_token"));

        let res = req
            .send().await?
            .text().await?;

        Ok(res)
    }

    pub async fn get_weekly_report(&self, query: &Query) -> Result<WeeklyReport> {
        const ENDPOINT: &str = "https://api.track.toggl.com/reports/api/v2/weekly";

        let res = self.rest_get(ENDPOINT, query).await?;
        let report: WeeklyReport = serde_json::from_str(&res)?;

        Ok(report)
    }

    pub async fn get_detailed_report(&self, query: &Query) -> Result<DetailedReport> {
        const ENDPOINT: &str = "https://api.track.toggl.com/reports/api/v2/details";

        let res = self.rest_get(ENDPOINT, query).await?;
        let report: DetailedReport = serde_json::from_str(&res)?;

        Ok(report)
    }

    pub async fn get_summary_report(&self, query: &Query) -> Result<SummaryReport> {
        const ENDPOINT: &str = "https://api.track.toggl.com/reports/api/v2/summary";

        let res = self.rest_get(ENDPOINT, query).await?;
        let report: SummaryReport = serde_json::from_str(&res)?;

        Ok(report)
    }
}
