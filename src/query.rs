//! Data structures to construct queries.
//!
//! For more details, see https://github.com/toggl/toggl_api_docs/blob/master/reports.md.

use std::fmt::{self, Display};
use chrono::{Date, Utc};
use itertools::Itertools;

#[derive(Default, Debug)]
pub struct Query {
    workspace_id: u64,
    grouping: Option<GroupingKind>,
    subgrouping: Option<GroupingKind>,
    subgrouping_ids: Option<bool>,
    grouped_time_entry_ids: Option<bool>,
    calculate: Option<CalculateKind>,
    since: Option<Date<Utc>>,
    until: Option<Date<Utc>>,
    page: Option<u64>,
    billable: Option<BillableKind>,
    client_ids: Option<Vec<u64>>,
    project_ids: Option<Vec<u64>>,
    user_ids: Option<Vec<u64>>,
    members_of_group_ids: Option<Vec<u64>>,
    or_members_of_group_ids: Option<Vec<u64>>,
    tag_ids: Option<Vec<u64>>,
    task_ids: Option<Vec<u64>>,
    time_entry_ids: Option<Vec<u64>>,
    description: Option<String>,
    without_description: Option<bool>,
    order_field: Option<OrderKind>,
    order_desc: Option<bool>,
    distinct_rates: Option<bool>,
    rounding: Option<bool>,
    display_hours: Option<DisplayHoursKind>,
}

macro_rules! define_setter {
    ($mem:ident, $type:ty) => {
        pub fn $mem(self, val: $type) -> Self {
            Self {
                $mem: Some(val),
                ..self
            }
        }
    }
}

macro_rules! push_query {
    ($self:ident, $query_vec:ident, $mem:ident) => {
        if let Some($mem) = &$self.$mem {
            $query_vec.push((stringify!($mem), $mem.to_string().to_lowercase()));
        }
    }
}

macro_rules! push_query_vec {
    ($self:ident, $query_vec:ident, $mem:ident) => {
        if let Some($mem) = &$self.$mem {
            $query_vec.push((stringify!($mem), $mem.iter().join(",")));
        }
    }
}

macro_rules! push_query_onoff {
    ($self:ident, $query_vec:ident, $mem:ident) => {
        match &$self.$mem {
            Some(true) => $query_vec.push((stringify!($mem), String::from("on"))),
            Some(false) => $query_vec.push((stringify!($mem), String::from("off"))),
            _ => ()
        }
    }
}

impl Query {
    pub fn new(workspace_id: u64) -> Self {
        Self {
            workspace_id,
            ..Default::default()
        }
    }

    define_setter!(grouping, GroupingKind);
    define_setter!(subgrouping, GroupingKind);
    define_setter!(subgrouping_ids, bool);
    define_setter!(grouped_time_entry_ids, bool);
    define_setter!(calculate, CalculateKind);
    define_setter!(since, Date<Utc>);
    define_setter!(until, Date<Utc>);
    define_setter!(page, u64);
    define_setter!(billable, BillableKind);
    define_setter!(client_ids, Vec<u64>);
    define_setter!(project_ids, Vec<u64>);
    define_setter!(user_ids, Vec<u64>);
    define_setter!(members_of_group_ids, Vec<u64>);
    define_setter!(or_members_of_group_ids, Vec<u64>);
    define_setter!(tag_ids, Vec<u64>);
    define_setter!(task_ids, Vec<u64>);
    define_setter!(time_entry_ids, Vec<u64>);
    define_setter!(description, String);
    define_setter!(without_description, bool);
    define_setter!(order_field, OrderKind);
    define_setter!(order_desc, bool);
    define_setter!(distinct_rates, bool);
    define_setter!(rounding, bool);
    define_setter!(display_hours, DisplayHoursKind);

    pub fn to_vec(&self) -> Vec<(&'static str, String)> {
        let mut query_vec = Vec::new();

        query_vec.push(("workspace_id", self.workspace_id.to_string()));
        push_query!(self, query_vec, grouping);
        push_query!(self, query_vec, subgrouping);
        push_query!(self, query_vec, subgrouping_ids);
        push_query!(self, query_vec, grouped_time_entry_ids);
        push_query!(self, query_vec, calculate);
        /* since */
        /* until */
        push_query!(self, query_vec, page);
        push_query!(self, query_vec, billable);
        push_query_vec!(self, query_vec, client_ids);
        push_query_vec!(self, query_vec, project_ids);
        push_query_vec!(self, query_vec, user_ids);
        push_query_vec!(self, query_vec, members_of_group_ids);
        push_query_vec!(self, query_vec, or_members_of_group_ids);
        push_query_vec!(self, query_vec, tag_ids);
        push_query_vec!(self, query_vec, task_ids);
        push_query_vec!(self, query_vec, time_entry_ids);
        push_query!(self, query_vec, description);
        push_query!(self, query_vec, without_description);
        push_query!(self, query_vec, order_field);
        push_query_onoff!(self, query_vec, order_desc);
        push_query_onoff!(self, query_vec, distinct_rates);
        push_query_onoff!(self, query_vec, rounding);
        push_query!(self, query_vec, display_hours);

        if let Some(since) = &self.since {
            query_vec.push(("since", since.format("%F").to_string()));
        }
        if let Some(until) = &self.until {
            query_vec.push(("until", until.format("%F").to_string()));
        }

        query_vec
    }
}

#[derive(Debug)]
pub enum GroupingKind {
    Projects,
    Clients,
    Users,
    Tasks,
    TimeEntries,
}

impl fmt::Display for GroupingKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupingKind::Projects => write!(f, "projects"),
            GroupingKind::Clients => write!(f, "clients"),
            GroupingKind::Users => write!(f, "users"),
            GroupingKind::Tasks => write!(f, "tasks"),
            GroupingKind::TimeEntries => write!(f, "time_entries"),
        }
    }
}

#[derive(Debug, Display)]
pub enum CalculateKind {
    Time,
    Earnings
}

#[derive(Debug, Display)]
pub enum BillableKind {
    Yes,
    No,
    Both,
}

#[derive(Debug)]
pub enum OrderKind {
    Date,
    Description,
    Duration,
    User,
    Title,
    Amount,
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    WeekTotal,
}

impl fmt::Display for OrderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderKind::Date => write!(f, "date"),
            OrderKind::Description => write!(f, "description"),
            OrderKind::Duration => write!(f, "duration"),
            OrderKind::User => write!(f, "user"),
            OrderKind::Title => write!(f, "title"),
            OrderKind::Amount => write!(f, "amount"),
            OrderKind::Day1 => write!(f, "day1"),
            OrderKind::Day2 => write!(f, "day2"),
            OrderKind::Day3 => write!(f, "day3"),
            OrderKind::Day4 => write!(f, "day4"),
            OrderKind::Day5 => write!(f, "day5"),
            OrderKind::Day6 => write!(f, "day6"),
            OrderKind::Day7 => write!(f, "day7"),
            OrderKind::WeekTotal => write!(f, "week_total"),
        }
    }
}

#[derive(Debug, Display)]
pub enum DisplayHoursKind {
    Decimal,
    Minites,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let query = Query::new(334);
        let vec = query.to_vec();

        assert_eq!(vec, vec![("workspace_id", String::from("334"))]);
    }

    #[test]
    fn normal_option() {
        let query = Query::new(334)
            .grouping(GroupingKind::Projects)
            .subgrouping(GroupingKind::TimeEntries)
            .subgrouping_ids(true)
            .grouped_time_entry_ids(true)
            .calculate(CalculateKind::Earnings)
            .page(264)
            .billable(BillableKind::No)
            .description(String::from("desc"))
            .without_description(false)
            .order_field(OrderKind::Day7)
            .display_hours(DisplayHoursKind::Minites);
        let vec = query.to_vec();

        let ans = vec![
            ("workspace_id", String::from("334")),
            ("grouping", String::from("projects")),
            ("subgrouping", String::from("time_entries")),
            ("subgrouping_ids", String::from("true")),
            ("grouped_time_entry_ids", String::from("true")),
            ("calculate", String::from("earnings")),
            ("page", String::from("264")),
            ("billable", String::from("no")),
            ("description", String::from("desc")),
            ("without_description", String::from("false")),
            ("order_field", String::from("day7")),
            ("display_hours", String::from("minites"))
        ];

        assert_eq!(vec, ans);
    }

    #[test]
    fn vec_option() {
        let query = Query::new(334)
            .client_ids(vec![3, 3, 4])
            .project_ids(vec![2, 6, 4])
            .user_ids(vec![2, 2, 7])
            .members_of_group_ids(vec![33, 4])
            .or_members_of_group_ids(vec![26, 4])
            .tag_ids(vec![22, 7])
            .task_ids(vec![3, 34])
            .time_entry_ids(vec![2, 64]);
        let vec = query.to_vec();

        let ans = vec![
            ("workspace_id", String::from("334")),
            ("client_ids", String::from("3,3,4")),
            ("project_ids", String::from("2,6,4")),
            ("user_ids", String::from("2,2,7")),
            ("members_of_group_ids", String::from("33,4")),
            ("or_members_of_group_ids", String::from("26,4")),
            ("tag_ids", String::from("22,7")),
            ("task_ids", String::from("3,34")),
            ("time_entry_ids", String::from("2,64"))
        ];

        assert_eq!(vec, ans);
    }

    #[test]
    fn vec_onoff() {
        let query = Query::new(334)
            .order_desc(false)
            .distinct_rates(false)
            .rounding(false);
        let vec = query.to_vec();

        let ans = vec![
            ("workspace_id", String::from("334")),
            ("order_desc", String::from("off")),
            ("distinct_rates", String::from("off")),
            ("rounding", String::from("off")),
        ];

        assert_eq!(vec, ans);
    }

    #[test]
    fn vec_special() {
        use chrono::{DateTime, Utc};

        let dt = DateTime::parse_from_str("2021/02/28 19:59:59 +0000", "%Y/%m/%d %H:%M:%S %z").unwrap();
        let date = dt.with_timezone(&Utc).date();
        let query = Query::new(334)
            .since(date)
            .until(date);
        let vec = query.to_vec();

        let ans = vec![
            ("workspace_id", String::from("334")),
            ("since", String::from("2021-02-28")),
            ("until", String::from("2021-02-28")),
        ];

        assert_eq!(vec, ans);
    }
}
