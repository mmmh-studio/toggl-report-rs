//! Data structures to store results of reports

use chrono::Duration;
use serde::{Deserialize, Deserializer};
use std::convert::{AsRef, AsMut};
use std::ops::{Deref, DerefMut};

/// A type to represent total times of a week
pub type TimeTotals = [DurationWrapper; 8];

/// Deserializer for chrono::Duration
fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where D: Deserializer<'de>
{
    let opt: Option<i64> = Option::deserialize(deserializer)?;
    Ok(opt
        .map(chrono::Duration::milliseconds)
        .unwrap_or(Duration::seconds(0))
    )
}

/// Deserializer for amount of money
fn deserialize_earning<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let opt: Option<f64> = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// Deserializer for Option<T>
fn deserialize_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where D: Deserializer<'de>,
          T: Deserialize<'de>
{
    Deserialize::deserialize(deserializer)
}

/// A data structure to wrap chrono::Duration to deserialize nullable json
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct DurationWrapper(
    #[serde(deserialize_with="deserialize_duration")]
    Duration
);

impl Deref for DurationWrapper {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DurationWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<DurationWrapper> for Duration {
    fn from(from: DurationWrapper) -> Duration {
        from.0
    }
}

impl AsRef<Duration> for DurationWrapper {
    fn as_ref(&self) -> &Duration {
        &self.0
    }
}

impl AsMut<Duration> for DurationWrapper {
    fn as_mut(&mut self) -> &mut Duration {
        &mut self.0
    }
}

/// A data structure to wrap chrono::Duration to deserialize nullable json
#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct EarningWrapper (
    #[serde(deserialize_with="deserialize_earning")]
    f64
);

impl Deref for EarningWrapper {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EarningWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<EarningWrapper> for f64 {
    fn from(from: EarningWrapper) -> f64 {
        from.0
    }
}

impl AsRef<f64> for EarningWrapper {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl AsMut<f64> for EarningWrapper {
    fn as_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

/// A type to represent total earnings of a week
#[derive(Debug, Deserialize)]
pub struct EarningTotals {
    pub currency: Option<String>,
    pub amount: [EarningWrapper; 8],
}

/// A structure to sotre currency
#[derive(Debug, Deserialize)]
pub struct Currency {
    pub currency: Option<String>,
    pub amount: EarningWrapper,
}

/// A generic structure to store response from Toggl
#[derive(Debug, Deserialize)]
pub struct Report<Data> {
    pub total_grand: DurationWrapper,
    pub total_billable: EarningWrapper,
    pub total_currencies: Vec<Currency>,
    pub data: Vec<Data>,
}

/// A structure to represent Title entries
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Title {
    ProjectTitle (
        ProjectTitle
    ),
    ClientTitle (
        ClientTitle
    ),
    UserTitle (
        UserTitle
    ),
    TaskTitle (
        TaskTitle
    ),
    TimeEntryTitle (
        TimeEntryTitle
    )
}

/// A string to represent null entries
const NONE_STR: &'static str= "(none)";

impl Title {
    /// Convert to String
    pub fn name(&self) -> String {
        match self {
            Title::ProjectTitle(title) => title.name(),
            Title::ClientTitle(title) => title.name(),
            Title::UserTitle(title) => title.name(),
            Title::TaskTitle(title) => title.name(),
            Title::TimeEntryTitle(title) => title.name()
        }
    }
}

/// A structure to represent title entries of projects
#[derive(Debug, Deserialize)]
pub struct ProjectTitle {
    #[serde(deserialize_with="deserialize_option")]
    pub project: Option<String>,
    #[serde(deserialize_with="deserialize_option")]
    pub client: Option<String>,
}

impl ProjectTitle {
    fn name(&self) -> String {
        self.project.clone()
            .unwrap_or(NONE_STR.to_owned())
    }
}

/// A structure to represent title entries of clients
#[derive(Debug, Deserialize)]
pub struct ClientTitle {
    #[serde(deserialize_with="deserialize_option")]
    pub client: Option<String>,
}

impl ClientTitle {
    pub fn name(&self) -> String {
        self.client.clone()
            .unwrap_or(NONE_STR.to_owned())
    }
}

/// A structure to represent title entries of users
#[derive(Debug, Deserialize)]
pub struct UserTitle {
    #[serde(deserialize_with="deserialize_option")]
    pub user: Option<String>,
}

impl UserTitle {
    pub fn name(&self) -> String {
        self.user.clone()
            .unwrap_or(NONE_STR.to_owned())
    }
}

/// A structure to represent title entries of tasks
#[derive(Debug, Deserialize)]
pub struct TaskTitle {
    #[serde(deserialize_with="deserialize_option")]
    pub task: Option<String>,
}

impl TaskTitle {
    pub fn name(&self) -> String {
        self.task.clone()
            .unwrap_or(NONE_STR.to_owned())
    }
}

/// A structure to represent title entries of time entries
#[derive(Debug, Deserialize)]
pub struct TimeEntryTitle {
    #[serde(deserialize_with="deserialize_option")]
    pub time_entry: Option<String>,
}

impl TimeEntryTitle {
    pub fn name(&self) -> String {
        self.time_entry.clone()
            .unwrap_or(NONE_STR.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, Eq, PartialEq)]
    struct Hoge {
        duration: DurationWrapper
    }

    #[test]
    fn duration_basic() {
        let json = r#"{ "duration": 334 }"#;
        let result: Hoge = serde_json::from_str(&json).unwrap();
        let ans = Hoge { duration: DurationWrapper ( Duration::milliseconds(334) ) };

        assert_eq!(ans, result);
    }

    #[test]
    fn duration_null() {
        let json = r#"{ "duration": null }"#;
        let result: Hoge = serde_json::from_str(&json).unwrap();
        let ans = Hoge { duration: DurationWrapper ( Duration::milliseconds(0) ) };

        assert_eq!(ans, result);
    }

    #[test]
    fn duration_deref() {
        let wrapper = DurationWrapper(Duration::milliseconds(334));
        let ans = Duration::milliseconds(334);
        let into_val: Duration = wrapper.into();

        assert_eq!(ans.to_string(), wrapper.to_string());
        assert_eq!(ans, into_val);
        assert_eq!(&ans, wrapper.as_ref());
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Fuga {
        earning: EarningWrapper,
    }

    #[test]
    fn earning_basic() {
        let json = r#"{"earning": 33.4}"#;
        let result: Fuga = serde_json::from_str(&json).unwrap();
        let ans = Fuga { earning: EarningWrapper(33.4) };

        assert_eq!(ans, result);
    }

    #[test]
    fn earning_null() {
        let json = r#"{"earning": null}"#;
        let result: Fuga = serde_json::from_str(&json).unwrap();
        let ans = Fuga { earning: EarningWrapper(0.0) };

        assert_eq!(ans, result);
    }

    #[test]
    fn earning_deref() {
        let wrapper = EarningWrapper(334.0);
        let ans = 334f64;
        let into_val: f64 = wrapper.into();

        assert_eq!(ans.to_string(), wrapper.to_string());
        assert_eq!(ans, into_val);
        assert_eq!(&ans, wrapper.as_ref());
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Poyo {
        #[serde(deserialize_with="deserialize_option")]
        op: Option<u64>,
    }

    #[test]
    fn option_basic() {
        let json = r#"{"op": 334}"#;
        let result: Poyo = serde_json::from_str(&json).unwrap();
        let ans = Poyo { op: Some(334) };

        assert_eq!(ans, result);
    }

    #[test]
    fn option_null() {
        let json = r#"{"op": null}"#;
        let result: Poyo = serde_json::from_str(&json).unwrap();
        let ans = Poyo { op: None };

        assert_eq!(ans, result);
    }

    #[test]
    #[should_panic]
    fn option_illegal() {
        let json = r#"{}"#;
        let _: Poyo = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn title_name_basic() {
        let project = Title::ProjectTitle(ProjectTitle {
            project: Some(String::from("proj")),
            client: None,
        });
        assert_eq!(project.name(), String::from("proj"));

        let client = Title::ClientTitle(ClientTitle {
            client: Some(String::from("cli")),
        });
        assert_eq!(client.name(), String::from("cli"));

        let user = Title::UserTitle(UserTitle {
            user: Some(String::from("us")),
        });
        assert_eq!(user.name(), String::from("us"));

        let task = Title::TaskTitle(TaskTitle {
            task: Some(String::from("tas")),
        });
        assert_eq!(task.name(), String::from("tas"));

        let time_entry = Title::TimeEntryTitle(TimeEntryTitle {
            time_entry: Some(String::from("te")),
        });
        assert_eq!(time_entry.name(), String::from("te"));
    }

    #[test]
    fn title_name_none() {
        let project = Title::ProjectTitle(ProjectTitle {
            project: None,
            client: None,
        });
        assert_eq!(project.name(), NONE_STR.to_owned());

        let client = Title::ClientTitle(ClientTitle {
            client: None,
        });
        assert_eq!(client.name(), NONE_STR.to_owned());

        let user = Title::UserTitle(UserTitle {
            user: None,
        });
        assert_eq!(user.name(), NONE_STR.to_owned());

        let task = Title::TaskTitle(TaskTitle {
            task: None,
        });
        assert_eq!(task.name(), NONE_STR.to_owned());

        let time_entry = Title::TimeEntryTitle(TimeEntryTitle {
            time_entry: None,
        });
        assert_eq!(time_entry.name(), NONE_STR.to_owned());
    }
}